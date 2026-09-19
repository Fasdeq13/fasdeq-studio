use crate::editor::diagnostics::Diagnostic;

#[derive(Clone, Debug)]
pub struct QuickFix {
    pub title: String,
    pub replacement: Option<String>,
}

pub fn suggest_fixes(diagnostic: &Diagnostic, line_text: &str) -> Vec<QuickFix> {
    let msg = diagnostic.message.to_lowercase();
    let mut fixes = Vec::new();

    if msg.contains("expected `;`") || msg.contains("expected ;") || msg.contains("missing terminating") {
        fixes.push(QuickFix {
            title: "Add missing semicolon".to_string(),
            replacement: Some(format!("{}{}", line_text.trim_end(), ";")),
        });
    }

    if msg.contains("unused variable") || msg.contains("unused import") {
        if let Some(name) = extract_backtick_token(&diagnostic.message) {
            fixes.push(QuickFix {
                title: format!("Prefix `{name}` with an underscore"),
                replacement: Some(line_text.replacen(&name, &format!("_{name}"), 1)),
            });
        }
        fixes.push(QuickFix {
            title: "Remove this line".to_string(),
            replacement: Some(String::new()),
        });
    }

    if msg.contains("mismatched types") && msg.contains("expected") && msg.contains("found") {
        fixes.push(QuickFix {
            title: "Review the expected type at this location".to_string(),
            replacement: None,
        });
    }

    if msg.contains("cannot find value") || msg.contains("cannot find function") || msg.contains("undeclared") {
        if let Some(name) = extract_backtick_token(&diagnostic.message) {
            fixes.push(QuickFix {
                title: format!("Check the spelling of `{name}`"),
                replacement: None,
            });
        }
    }

    if msg.contains("implicit declaration of function") {
        fixes.push(QuickFix {
            title: "Add a matching #include for this function".to_string(),
            replacement: None,
        });
    }

    if msg.contains("expected `)`") || msg.contains("expected )") {
        fixes.push(QuickFix {
            title: "Add missing closing parenthesis".to_string(),
            replacement: Some(format!("{})", line_text.trim_end())),
        });
    }

    if msg.contains("expected `}`") || msg.contains("expected }") {
        fixes.push(QuickFix {
            title: "Add missing closing brace".to_string(),
            replacement: Some(format!("{}}}", line_text.trim_end())),
        });
    }

    if msg.contains("borrow") && msg.contains("mutable") {
        fixes.push(QuickFix {
            title: "Consider cloning the value instead of borrowing it".to_string(),
            replacement: None,
        });
    }

    if fixes.is_empty() {
        fixes.push(QuickFix {
            title: "No automatic fix available for this diagnostic".to_string(),
            replacement: None,
        });
    }

    fixes
}

fn extract_backtick_token(message: &str) -> Option<String> {
    let start = message.find('`')?;
    let rest = &message[start + 1..];
    let end = rest.find('`')?;
    Some(rest[..end].to_string())
}
