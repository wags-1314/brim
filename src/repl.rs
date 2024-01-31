use reedline::{Prompt, DefaultValidator, Reedline, Signal, default_emacs_keybindings, KeyModifiers, KeyCode, ReedlineEvent, EditCommand, Emacs};
use std::borrow::Cow;
use crate::lexer::Lexer;

#[derive(Clone)]
struct BrimPrompt;

impl Prompt for BrimPrompt {
    fn render_prompt_left(&self) -> Cow<str> {
	Cow::Owned("".to_owned())
    }

    fn render_prompt_right(&self) -> Cow<str> {
        Cow::Owned("".to_owned())
    }

    fn render_prompt_indicator(&self, prompt_mode: reedline::PromptEditMode) -> Cow<str> {
        Cow::Owned(">>> ".to_owned())
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<str> {
        Cow::Owned("... ".to_owned())
    }

    fn render_prompt_history_search_indicator(
        &self,
        history_search: reedline::PromptHistorySearch,
    ) -> Cow<str> {
	Cow::Owned("".to_owned())
    }
}

pub fn repl() {
    let validator = Box::new(DefaultValidator);
    let prompt = BrimPrompt{};
    let mut keybindings = default_emacs_keybindings();
    keybindings.remove_binding(KeyModifiers::NONE, KeyCode::Tab);
    keybindings.add_binding(
	KeyModifiers::NONE,
	KeyCode::Tab,
	ReedlineEvent::Edit(vec![EditCommand::InsertChar('\t')])
    );
    let mut line_editor = Reedline::create()
        .with_validator(validator)
        .with_edit_mode(Box::new(Emacs::new(keybindings)));

    println!("== Brim v1.0 ==");
    println!("ctrl + d to exit");
    loop {
	let sig = line_editor.read_line(&prompt);
	match sig {
	    Ok(Signal::Success(buffer)) => {
		let lexer = Lexer::new(&buffer);
		for token in lexer {
		    println!("{:?}", token);
		}
	    }
	    Ok(Signal::CtrlC) => {
		continue;
	    }
	    _ => {
		break;
	    }
	}
    }
}
