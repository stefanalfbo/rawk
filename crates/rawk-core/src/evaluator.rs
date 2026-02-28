use crate::{
    Action, Program, Rule,
    ast::{Expression, Statement},
    token::TokenKind,
};
use std::collections::HashMap;
use std::cell::Cell;

pub struct Evaluator<'a> {
    program: Program<'a>,
    input_lines: Vec<String>,
    current_line_number: Cell<usize>,
    current_line: Option<String>,
    field_separator: String,
    output_field_separator: String,
    current_filename: String,
    variables: HashMap<String, String>,
}

impl<'a> Evaluator<'a> {
    pub fn new(program: Program<'a>, input_lines: Vec<String>) -> Self {
        Self {
            program,
            input_lines,
            current_line_number: Cell::new(0),
            current_line: None,
            field_separator: " ".to_string(),
            output_field_separator: " ".to_string(),
            current_filename: "onetrueawk-testdata/countries".to_string(),
            variables: HashMap::new(),
        }
    }

    pub fn eval(&mut self) -> Vec<String> {
        let mut output_lines: Vec<String> = Vec::new();

        let begin_rules: Vec<Rule<'a>> = self.program.begin_blocks_iter().cloned().collect();
        for rule in begin_rules.iter() {
            output_lines.extend(self.eval_begin_rule(rule));
        }

        let rules: Vec<Rule<'a>> = self.program.rules_iter().cloned().collect();
        for rule in rules.iter() {
            output_lines.extend(self.eval_rule(rule));
        }

        self.current_line_number.set(self.input_lines.len());
        self.current_line = None;

        let end_rules: Vec<Rule<'a>> = self.program.end_blocks_iter().cloned().collect();
        for rule in end_rules.iter() {
            output_lines.extend(self.eval_end_rule(rule));
        }

        output_lines
    }

    fn eval_rule(&mut self, rule: &Rule) -> Vec<String> {
        let mut output_lines = Vec::new();
        let mut range_active = false;

        let input_lines = self.input_lines.clone();
        for (i, input_line) in input_lines.iter().enumerate() {
            self.current_line_number.set(i + 1);
            self.current_line = Some(input_line.clone());

            match rule {
                Rule::Action(action) => output_lines.extend(self.eval_action(action, Some(input_line))),
                Rule::PatternAction { pattern, action } => {
                    let matches = match pattern.as_ref() {
                        Some(expr) => self.eval_pattern_condition(expr, &mut range_active),
                        None => true,
                    };
                    if matches {
                        if let Some(action) = action {
                            output_lines.extend(self.eval_action(action, Some(input_line)));
                        } else {
                            output_lines.push(input_line.clone());
                        }
                    }
                }
                _ => {}
            }
        }

        self.current_line = None;
        output_lines
    }

    fn eval_pattern_condition(
        &self,
        expression: &Expression<'_>,
        range_active: &mut bool,
    ) -> bool {
        if let Expression::Infix {
            left,
            operator,
            right,
        } = expression
        {
            if operator.kind == TokenKind::Comma {
                if !*range_active {
                    let start = self.eval_condition(left);
                    if !start {
                        return false;
                    }
                    *range_active = true;
                }

                let matched = true;
                if self.eval_condition(right) {
                    *range_active = false;
                }
                return matched;
            }
        }

        self.eval_condition(expression)
    }

    fn eval_begin_rule(&mut self, rule: &Rule) -> Vec<String> {
        match rule {
            Rule::Begin(action) => self.eval_action(action, None),
            _ => Vec::new(),
        }
    }

    fn eval_end_rule(&mut self, rule: &Rule) -> Vec<String> {
        match rule {
            Rule::End(action) => self.eval_action(action, None),
            _ => Vec::new(),
        }
    }

    fn eval_action(&mut self, action: &Action, input_line: Option<&str>) -> Vec<String> {
        let mut output = Vec::new();

        for statement in &action.statements {
            if let Some(line) = self.eval_statement(statement, input_line) {
                output.push(line);
            }
        }

        output
    }

    fn eval_statement(&mut self, statement: &Statement<'_>, input_line: Option<&str>) -> Option<String> {
        match statement {
            Statement::Print(expressions) => Some(self.eval_print(expressions, input_line)),
            Statement::Printf(expressions) => Some(self.eval_printf(expressions)),
            Statement::Gsub {
                pattern,
                replacement,
            } => {
                self.eval_gsub(pattern, replacement);
                None
            }
            Statement::Assignment { identifier, value } => {
                self.eval_assignment(identifier, value);
                None
            }
            Statement::FieldAssignment { field, value } => {
                self.eval_field_assignment(field, value);
                None
            }
            Statement::AddAssignment { identifier, value } => {
                self.eval_add_assignment(identifier, value);
                None
            }
            Statement::PreIncrement { identifier } => {
                self.eval_pre_increment(identifier);
                None
            }
        }
    }

    fn eval_print(&self, expressions: &[Expression<'_>], input_line: Option<&str>) -> String {
        if expressions.is_empty() {
            return self
                .current_line
                .as_deref()
                .or(input_line)
                .unwrap_or("")
                .to_string();
        }

        let parts = expressions
            .iter()
            .map(|expr| self.eval_expression(expr))
            .collect::<Vec<String>>();
        parts.join(&self.output_field_separator)
    }

    fn eval_printf(&self, expressions: &[Expression<'_>]) -> String {
        if expressions.is_empty() {
            return String::new();
        }

        let format = self.eval_expression(&expressions[0]);
        let format = unescape_awk_string(&format);
        let args: Vec<String> = expressions
            .iter()
            .skip(1)
            .map(|expr| self.eval_expression(expr))
            .collect();

        let rendered = expand_tabs(&format_printf(&format, &args));
        rendered.trim_end_matches(['\r', '\n']).to_string()
    }

    fn eval_assignment(&mut self, identifier: &str, value: &Expression<'_>) {
        let assigned_value = if let Expression::Infix {
            left,
            operator,
            right,
        } = value
        {
            if operator.kind == TokenKind::Assign {
                self.eval_assignment_infix(left, right)
            } else {
                self.eval_expression(value)
            }
        } else {
            self.eval_expression(value)
        };

        self.set_special_variable(identifier, &assigned_value);
        self.variables.insert(identifier.to_string(), assigned_value);
    }

    fn eval_add_assignment(&mut self, identifier: &str, value: &Expression<'_>) {
        let current = self
            .eval_identifier_expression(identifier)
            .parse::<f64>()
            .unwrap_or(0.0);
        let increment = self.eval_numeric_expression(value).unwrap_or(0.0);
        self.variables
            .insert(identifier.to_string(), (current + increment).to_string());
    }

    fn eval_field_assignment(&mut self, field: &Expression<'_>, value: &Expression<'_>) {
        let line = match self.current_line.as_ref() {
            Some(value) => value.clone(),
            None => return,
        };

        let index = self.eval_numeric_expression(field).unwrap_or(0.0) as i64;
        if index <= 0 {
            return;
        }

        let mut fields = self.split_fields(&line);
        while fields.len() < index as usize {
            fields.push(String::new());
        }
        fields[(index - 1) as usize] = self.eval_expression(value);
        self.current_line = Some(fields.join(&self.output_field_separator));
    }

    fn eval_assignment_infix(&mut self, left: &Expression<'_>, right: &Expression<'_>) -> String {
        let identifier = match left {
            Expression::Identifier(identifier) => *identifier,
            _ => return self.eval_expression(right),
        };

        let assigned_value = if let Expression::Infix {
            left: nested_left,
            operator: nested_operator,
            right: nested_right,
        } = right
        {
            if nested_operator.kind == TokenKind::Assign {
                self.eval_assignment_infix(nested_left, nested_right)
            } else {
                self.eval_expression(right)
            }
        } else {
            self.eval_expression(right)
        };

        self.set_special_variable(identifier, &assigned_value);
        self.variables
            .insert(identifier.to_string(), assigned_value.clone());
        assigned_value
    }

    fn set_special_variable(&mut self, identifier: &str, value: &str) {
        if identifier == "FS" {
            self.field_separator = unescape_awk_string(value);
        } else if identifier == "OFS" {
            self.output_field_separator = unescape_awk_string(value);
        }
    }

    fn eval_pre_increment(&mut self, identifier: &str) {
        let current = self
            .eval_identifier_expression(identifier)
            .parse::<f64>()
            .unwrap_or(0.0);
        self.variables
            .insert(identifier.to_string(), (current + 1.0).to_string());
    }

    fn eval_gsub(&mut self, pattern: &Expression<'_>, replacement: &Expression<'_>) {
        let line = match self.current_line.as_ref() {
            Some(value) => value.clone(),
            None => return,
        };

        let pattern = match pattern {
            Expression::Regex(value) => value.to_string(),
            _ => self.eval_expression(pattern),
        };
        let replacement = unescape_awk_string(&self.eval_expression(replacement));
        let replaced = awk_gsub_replace_all(&line, &pattern, &replacement);
        self.current_line = Some(replaced);
    }

    fn eval_expression(&self, expression: &Expression) -> String {
        match expression {
            Expression::String(value) => value.to_string(),
            Expression::Number(value) => format_awk_number(*value),
            Expression::Regex(value) => value.to_string(),
            Expression::Field(inner) => self.eval_field_expression(inner),
            Expression::Identifier(identifier) => self.eval_identifier_expression(identifier),
            Expression::Length(expression) => self.eval_length_expression(expression.as_deref()),
            Expression::Substr {
                string,
                start,
                length,
            } => self.eval_substr_expression(string, start, length.as_deref()),
            Expression::Concatenation { left, right } => {
                let mut value = self.eval_expression(left);
                value.push_str(&self.eval_expression(right));
                value
            }
            Expression::Infix {
                left,
                operator,
                right,
            } => self
                .eval_numeric_infix(left, operator, right)
                .map(format_awk_number)
                .unwrap_or_else(|| "not implemented".to_string()),
        }
    }

    fn eval_identifier_expression(&self, identifier: &str) -> String {
        match identifier {
            "NF" => {
                let line = match self.current_line.as_deref() {
                    Some(value) => value,
                    None => return "0".to_string(),
                };

                let field_count = self.split_fields(line).len();
                field_count.to_string()
            }
            "NR" => match self.current_line.as_ref() {
                Some(_) => self.current_line_number.get().to_string(),
                None => self.current_line_number.get().to_string(),
            },
            "FNR" => self.current_line_number.get().to_string(),
            "FILENAME" => self.current_filename.clone(),
            _ => self
                .variables
                .get(identifier)
                .cloned()
                .unwrap_or_default(),
        }
    }

    fn eval_field_expression(&self, expression: &Expression<'_>) -> String {
        let line = match self.current_line.as_deref() {
            Some(value) => value,
            None => return String::new(),
        };

        let index = match self.eval_numeric_expression(expression) {
            Some(value) => value as i64,
            None => return String::new(),
        };

        if index == 0 {
            return line.to_string();
        }

        if index < 0 {
            return String::new();
        }

        self.split_fields(line)
            .into_iter()
            .nth((index - 1) as usize)
            .unwrap_or_default()
    }

    fn eval_length_expression(&self, expression: Option<&Expression<'_>>) -> String {
        let value = match expression {
            Some(expr) => self.eval_expression(expr),
            None => self.current_line.clone().unwrap_or_default(),
        };
        value.chars().count().to_string()
    }

    fn eval_substr_expression(
        &self,
        string: &Expression<'_>,
        start: &Expression<'_>,
        length: Option<&Expression<'_>>,
    ) -> String {
        let source = self.eval_expression(string);
        let chars: Vec<char> = source.chars().collect();
        if chars.is_empty() {
            return String::new();
        }

        let start_index = self.eval_numeric_expression(start).unwrap_or(1.0) as i64;
        let start_index = if start_index <= 1 { 0 } else { (start_index - 1) as usize };
        if start_index >= chars.len() {
            return String::new();
        }

        let end_index = match length {
            Some(length) => {
                let len = self.eval_numeric_expression(length).unwrap_or(0.0) as i64;
                if len <= 0 {
                    return String::new();
                }
                (start_index + len as usize).min(chars.len())
            }
            None => chars.len(),
        };

        chars[start_index..end_index].iter().collect()
    }

    fn split_fields(&self, line: &str) -> Vec<String> {
        if self.field_separator == " " {
            line.split_whitespace().map(str::to_string).collect()
        } else {
            line.split(&self.field_separator)
                .map(str::to_string)
                .collect()
        }
    }

    fn eval_numeric_infix(
        &self,
        left: &Expression<'_>,
        operator: &crate::token::Token<'_>,
        right: &Expression<'_>,
    ) -> Option<f64> {
        let left_value = self.eval_numeric_expression(left)?;
        let right_value = self.eval_numeric_expression(right)?;

        match operator.kind {
            TokenKind::Plus => Some(left_value + right_value),
            TokenKind::Minus => Some(left_value - right_value),
            TokenKind::Asterisk => Some(left_value * right_value),
            TokenKind::Division => Some(left_value / right_value),
            TokenKind::Percent => Some(left_value % right_value),
            TokenKind::Caret => Some(left_value.powf(right_value)),
            TokenKind::GreaterThan => Some(if left_value > right_value { 1.0 } else { 0.0 }),
            TokenKind::GreaterThanOrEqual => {
                Some(if left_value >= right_value { 1.0 } else { 0.0 })
            }
            TokenKind::LessThan => Some(if left_value < right_value { 1.0 } else { 0.0 }),
            TokenKind::LessThanOrEqual => Some(if left_value <= right_value { 1.0 } else { 0.0 }),
            TokenKind::Equal => Some(if left_value == right_value { 1.0 } else { 0.0 }),
            TokenKind::NotEqual => Some(if left_value != right_value { 1.0 } else { 0.0 }),
            _ => None,
        }
    }

    fn eval_numeric_expression(&self, expression: &Expression<'_>) -> Option<f64> {
        match expression {
            Expression::Number(value) => Some(*value),
            Expression::Identifier(identifier) => self
                .eval_identifier_expression(identifier)
                .parse::<f64>()
                .ok()
                .or(Some(0.0)),
            Expression::Field(inner) => self.eval_field_expression(inner).parse::<f64>().ok(),
            Expression::Length(expression) => self
                .eval_length_expression(expression.as_deref())
                .parse::<f64>()
                .ok(),
            Expression::Concatenation { left, right } => {
                let mut value = self.eval_expression(left);
                value.push_str(&self.eval_expression(right));
                value.parse::<f64>().ok()
            }
            Expression::Infix {
                left,
                operator,
                right,
            } => self.eval_numeric_infix(left, operator, right),
            _ => None,
        }
    }

    fn eval_condition(&self, expression: &Expression<'_>) -> bool {
        if let Expression::Regex(pattern) = expression {
            return self
                .current_line
                .as_deref()
                .is_some_and(|line| awk_regex_matches(line, pattern));
        }

        if let Expression::Infix {
            left,
            operator,
            right,
        } = expression
        {
            if let Some(value) = self.eval_logical(left, operator.kind.clone(), right) {
                return value;
            }
            if let Some(value) = self.eval_regex_match(left, operator.kind.clone(), right) {
                return value;
            }
            if let Some(value) = self.eval_comparison(left, operator.kind.clone(), right) {
                return value;
            }
        }

        if let Some(value) = self.eval_numeric_expression(expression) {
            return value != 0.0;
        }

        let value = self.eval_expression(expression);
        !value.is_empty()
    }

    fn eval_comparison(
        &self,
        left: &Expression<'_>,
        operator: TokenKind,
        right: &Expression<'_>,
    ) -> Option<bool> {
        if !matches!(
            operator,
            TokenKind::Equal
                | TokenKind::NotEqual
                | TokenKind::GreaterThan
                | TokenKind::GreaterThanOrEqual
                | TokenKind::LessThan
                | TokenKind::LessThanOrEqual
        ) {
            return None;
        }

        let left_str = self.eval_expression(left);
        let right_str = self.eval_expression(right);
        let left_num = left_str.parse::<f64>().ok();
        let right_num = right_str.parse::<f64>().ok();

        let result = match (left_num, right_num) {
            (Some(l), Some(r)) => match operator {
                TokenKind::Equal => l == r,
                TokenKind::NotEqual => l != r,
                TokenKind::GreaterThan => l > r,
                TokenKind::GreaterThanOrEqual => l >= r,
                TokenKind::LessThan => l < r,
                TokenKind::LessThanOrEqual => l <= r,
                _ => unreachable!(),
            },
            _ => match operator {
                TokenKind::Equal => left_str == right_str,
                TokenKind::NotEqual => left_str != right_str,
                TokenKind::GreaterThan => left_str > right_str,
                TokenKind::GreaterThanOrEqual => left_str >= right_str,
                TokenKind::LessThan => left_str < right_str,
                TokenKind::LessThanOrEqual => left_str <= right_str,
                _ => unreachable!(),
            },
        };

        Some(result)
    }

    fn eval_regex_match(
        &self,
        left: &Expression<'_>,
        operator: TokenKind,
        right: &Expression<'_>,
    ) -> Option<bool> {
        if !matches!(operator, TokenKind::Tilde | TokenKind::NoMatch) {
            return None;
        }

        let haystack = self.eval_expression(left);
        let needle = match right {
            Expression::Regex(pattern) => pattern.to_string(),
            _ => self.eval_expression(right),
        };

        let matches = awk_regex_matches(&haystack, &needle);
        Some(if operator == TokenKind::NoMatch {
            !matches
        } else {
            matches
        })
    }

    fn eval_logical(
        &self,
        left: &Expression<'_>,
        operator: TokenKind,
        right: &Expression<'_>,
    ) -> Option<bool> {
        match operator {
            TokenKind::And => {
                if !self.eval_condition(left) {
                    Some(false)
                } else {
                    Some(self.eval_condition(right))
                }
            }
            TokenKind::Or => {
                if self.eval_condition(left) {
                    Some(true)
                } else {
                    Some(self.eval_condition(right))
                }
            }
            _ => None,
        }
    }
}

fn awk_regex_matches(text: &str, pattern: &str) -> bool {
    let anchored_start = pattern.starts_with('^');
    let anchored_end = pattern.ends_with('$');
    let mut core = pattern;

    if anchored_start {
        core = &core[1..];
    }
    if anchored_end && !core.is_empty() {
        core = &core[..core.len() - 1];
    }

    if core == "[0-9]+" {
        return match (anchored_start, anchored_end) {
            (true, true) => !text.is_empty() && text.chars().all(|c| c.is_ascii_digit()),
            (true, false) => text
                .chars()
                .next()
                .is_some_and(|c| c.is_ascii_digit()),
            (false, true) => text
                .chars()
                .last()
                .is_some_and(|c| c.is_ascii_digit()),
            (false, false) => text.chars().any(|c| c.is_ascii_digit()),
        };
    }

    if core.starts_with('(') && core.ends_with(')') && core.contains('|') {
        return core[1..core.len() - 1]
            .split('|')
            .any(|alt| match (anchored_start, anchored_end) {
            (true, true) => text == alt,
            (true, false) => text.starts_with(alt),
            (false, true) => text.ends_with(alt),
            (false, false) => text.contains(alt),
        });
    }

    if anchored_start && anchored_end {
        return text == core;
    }
    if anchored_start {
        return text.starts_with(core);
    }
    if anchored_end {
        return text.ends_with(core);
    }
    text.contains(core)
}

fn awk_gsub_replace_all(text: &str, pattern: &str, replacement: &str) -> String {
    let anchored_start = pattern.starts_with('^');
    let anchored_end = pattern.ends_with('$');
    let mut core = pattern;

    if anchored_start {
        core = &core[1..];
    }
    if anchored_end && !core.is_empty() {
        core = &core[..core.len() - 1];
    }
    if core.is_empty() {
        return text.to_string();
    }

    match (anchored_start, anchored_end) {
        (true, true) => {
            if text == core {
                replacement.to_string()
            } else {
                text.to_string()
            }
        }
        (true, false) => {
            if let Some(suffix) = text.strip_prefix(core) {
                format!("{replacement}{suffix}")
            } else {
                text.to_string()
            }
        }
        (false, true) => {
            if let Some(prefix) = text.strip_suffix(core) {
                format!("{prefix}{replacement}")
            } else {
                text.to_string()
            }
        }
        (false, false) => text.replace(core, replacement),
    }
}

fn unescape_awk_string(input: &str) -> String {
    let mut output = String::new();
    let mut chars = input.chars().peekable();

    while let Some(ch) = chars.next() {
        if ch != '\\' {
            output.push(ch);
            continue;
        }

        match chars.next() {
            Some('n') => output.push('\n'),
            Some('t') => output.push('\t'),
            Some('r') => output.push('\r'),
            Some('\\') => output.push('\\'),
            Some('"') => output.push('"'),
            Some(other) => {
                output.push('\\');
                output.push(other);
            }
            None => output.push('\\'),
        }
    }

    output
}

fn format_printf(format: &str, args: &[String]) -> String {
    let mut result = String::new();
    let mut chars = format.chars().peekable();
    let mut arg_index = 0usize;

    while let Some(ch) = chars.next() {
        if ch != '%' {
            result.push(ch);
            continue;
        }

        if chars.peek() == Some(&'%') {
            chars.next();
            result.push('%');
            continue;
        }

        let mut left_justify = false;
        if chars.peek() == Some(&'-') {
            left_justify = true;
            chars.next();
        }

        let mut width: usize = 0;
        while let Some(next) = chars.peek() {
            if next.is_ascii_digit() {
                width = (width * 10) + (*next as usize - '0' as usize);
                chars.next();
            } else {
                break;
            }
        }

        let mut precision: Option<usize> = None;
        if chars.peek() == Some(&'.') {
            chars.next();
            let mut value = 0usize;
            while let Some(next) = chars.peek() {
                if next.is_ascii_digit() {
                    value = (value * 10) + (*next as usize - '0' as usize);
                    chars.next();
                } else {
                    break;
                }
            }
            precision = Some(value);
        }

        let specifier = match chars.next() {
            Some(value) => value,
            None => {
                result.push('%');
                break;
            }
        };

        let arg = args.get(arg_index).cloned().unwrap_or_default();
        arg_index += 1;

        let formatted = match specifier {
            's' => arg,
            'd' => arg
                .parse::<f64>()
                .map(|value| value.trunc() as i64)
                .unwrap_or(0)
                .to_string(),
            'f' => {
                let value = arg.parse::<f64>().unwrap_or(0.0);
                let precision = precision.unwrap_or(6);
                format!("{value:.precision$}")
            }
            _ => {
                result.push('%');
                if left_justify {
                    result.push('-');
                }
                if width > 0 {
                    result.push_str(&width.to_string());
                }
                if let Some(precision) = precision {
                    result.push('.');
                    result.push_str(&precision.to_string());
                }
                result.push(specifier);
                continue;
            }
        };

        if width <= formatted.len() {
            result.push_str(&formatted);
            continue;
        }

        let padding_len = width - formatted.len();
        let padding = " ".repeat(padding_len);
        if left_justify {
            result.push_str(&formatted);
            result.push_str(&padding);
        } else {
            result.push_str(&padding);
            result.push_str(&formatted);
        }
    }

    result
}

fn expand_tabs(input: &str) -> String {
    expand_tabs_with_tabstop(input, 4)
}

fn expand_tabs_with_tabstop(input: &str, tabstop: usize) -> String {
    let mut output = String::new();
    let mut column = 0usize;

    for ch in input.chars() {
        if ch == '\t' {
            let spaces = tabstop - (column % tabstop);
            output.push_str(&" ".repeat(spaces));
            column += spaces;
            continue;
        }

        output.push(ch);
        if ch == '\n' || ch == '\r' {
            column = 0;
        } else {
            column += 1;
        }
    }

    output
}

fn format_awk_number(value: f64) -> String {
    if !value.is_finite() {
        return value.to_string();
    }

    if value == 0.0 {
        return "0".to_string();
    }

    let abs = value.abs();
    let digits_before_decimal = if abs >= 1.0 {
        abs.log10().floor() as i32 + 1
    } else {
        0
    };
    let decimals = (6 - digits_before_decimal).max(0) as usize;
    let formatted = format!("{value:.decimals$}");
    formatted
        .trim_end_matches('0')
        .trim_end_matches('.')
        .to_string()
}

#[cfg(test)]
mod tests {
    use crate::{Lexer, Parser};

    use super::*;

    #[test]
    fn eval_print_action_outputs_input_line() {
        let lexer = Lexer::new("{ print }");
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec!["hello, world!".to_string()]);

        let output = evaluator.eval();

        assert_eq!(output.len(), 1);
        assert_eq!(output[0], "hello, world!");
    }

    #[test]
    fn eval_begin_print_string_literal() {
        let lexer = Lexer::new(r#"BEGIN { print "hello" }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec![]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["hello".to_string()]);
    }

    #[test]
    fn eval_end_print_string_literal() {
        let lexer = Lexer::new(r#"END { print "42" } { print "hello" }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec!["one row".to_string()]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["hello".to_string(), "42".to_string()]);
    }

    #[test]
    fn eval_print_numeric_plus_infix_expression() {
        let lexer = Lexer::new(r#"BEGIN { print 1 + 2 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec![]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["3".to_string()]);
    }

    #[test]
    fn eval_print_numberic_multiply_infix_expression() {
        let lexer = Lexer::new(r#"BEGIN { print 2 * 3 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec![]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["6".to_string()]);
    }

    #[test]
    fn eval_print_numeric_mod_infix_expression() {
        let lexer = Lexer::new(r#"BEGIN { print 5 % 3 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec![]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["2".to_string()]);
    }

    #[test]
    fn eval_print_numeric_div_infix_expression() {
        let lexer = Lexer::new(r#"BEGIN { print 5 / 5 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec![]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["1".to_string()]);
    }

    #[test]
    fn eval_print_numeric_caret_infix_expression() {
        let lexer = Lexer::new(r#"BEGIN { print 2 ^ 3 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec![]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["8".to_string()]);
    }

    #[test]
    fn eval_print_numeric_minus_infix_expression() {
        let lexer = Lexer::new(r#"BEGIN { print 5 - 3 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec![]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["2".to_string()]);
    }

    #[test]
    fn eval_print_string_and_number_expressions() {
        let lexer = Lexer::new(r#"BEGIN { print "Value:" 42 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec![]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["Value:42".to_string()]);
    }

    #[test]
    fn eval_print_expression_with_parantheses() {
        let lexer = Lexer::new(r#"BEGIN { print (1 + 2) * 3 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec![]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["9".to_string()]);
    }

    #[test]
    fn eval_print_multiplication_has_higher_precedence_than_addition() {
        let lexer = Lexer::new(r#"BEGIN { print 1 + 2 * 3 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec![]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["7".to_string()]);
    }

    #[test]
    fn eval_print_power_is_right_associative() {
        let lexer = Lexer::new(r#"BEGIN { print 2 ^ 3 ^ 2 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec![]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["512".to_string()]);
    }

    #[test]
    fn eval_print_minus_is_left_associative() {
        let lexer = Lexer::new(r#"BEGIN { print 5 - 3 - 1 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec![]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["1".to_string()]);
    }

    #[test]
    fn eval_print_field_zero_returns_entire_line() {
        let lexer = Lexer::new(r#"{ print $0 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec!["one two".to_string()]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["one two".to_string()]);
    }

    #[test]
    fn eval_print_field_first_column() {
        let lexer = Lexer::new(r#"{ print $1, $3 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec!["one     two three".to_string()]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["one three".to_string()]);
    }

    #[test]
    fn eval_print_number_of_fields_identifier() {
        let lexer = Lexer::new(r#"{ print NF, $1 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec!["one two three".to_string()]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["3 one".to_string()]);
    }

    #[test]
    fn eval_print_number_of_fields_on_empty_line() {
        let lexer = Lexer::new(r#"{ print NF }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec!["".to_string()]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["0".to_string()]);
    }

    #[test]
    fn eval_print_uninitialized_identifier() {
        let lexer = Lexer::new(r#"{ print XYZ }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec!["one two".to_string()]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["".to_string()]);
    }

    #[test]
    fn eval_print_use_number_of_fields_in_field_expression() {
        let lexer = Lexer::new(r#"{ print $NF }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec!["one two three".to_string()]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["three".to_string()]);
    }

    #[test]
    fn eval_print_line_numbers() {
        let lexer = Lexer::new(r#"{ print NR, $0 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec!["one".to_string(), "two".to_string()]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["1 one".to_string(), "2 two".to_string()]);
    }

    #[test]
    fn eval_printf_with_width_and_alignment() {
        let lexer = Lexer::new(r#"{ printf "[%10s] [%-16d]\n", $1, $3 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec!["USSR 8649 275 Asia".to_string()]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["[      USSR] [275             ]".to_string()]);
    }

    #[test]
    fn eval_gsub_then_print_uses_updated_line() {
        let lexer = Lexer::new(r#"{ gsub(/USA/, "United States"); print }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec!["USA 3615 237 North America".to_string()]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["United States 3615 237 North America".to_string()]);
    }

    #[test]
    fn eval_print_length_and_line() {
        let lexer = Lexer::new(r#"{ print length, $0 }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec!["USSR 8649 275 Asia".to_string()]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["18 USSR 8649 275 Asia".to_string()]);
    }

    #[test]
    fn eval_field_assignment_with_substr_then_print() {
        let lexer = Lexer::new(r#"{ $1 = substr($1, 1, 3); print }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec!["Canada\t3852\t25\tNorth America".to_string()]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["Can 3852 25 North America".to_string()]);
    }

    #[test]
    fn eval_assignment_with_concatenation() {
        let lexer = Lexer::new(r#"{ s = s " " substr($1, 1, 3) } END { print s }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(
            program,
            vec![
                "USSR\t8649\t275\tAsia".to_string(),
                "Canada\t3852\t25\tNorth America".to_string(),
            ],
        );

        let output = evaluator.eval();

        assert_eq!(output, vec![" USS Can".to_string()]);
    }

    #[test]
    fn eval_chained_assignment_sets_fs_and_ofs() {
        let lexer = Lexer::new(r#"BEGIN { FS = OFS = "\t" } { $4 = "NA"; print }"#);
        let mut parser = Parser::new(lexer);
        let program = parser.parse_program();
        let mut evaluator = Evaluator::new(program, vec!["Canada\t3852\t25\tNorth America".to_string()]);

        let output = evaluator.eval();

        assert_eq!(output, vec!["Canada\t3852\t25\tNA".to_string()]);
    }
}
