use std::{collections::HashMap, iter::Peekable, str::Chars};

#[derive(Debug)]
enum State {
  Start,
  Key,
  Value,
  NextOrEnd,
  End,
}

#[derive(Debug,Clone)]
enum Json {
  String(Box<String>),
  Object(Box<JsonObject>),
  // Array(Vec<Box<Json>>)
}

#[derive(Debug,Clone)]
struct JsonObject {
  map: HashMap<String, Json>,
}

fn parser(json_string: &str) -> Result<JsonObject, String> {
  let mut char_itr = json_string.chars().peekable();
  let result = parser_recur(&mut char_itr);
  if result.is_ok() {
    println!("{:?}",char_itr.peek());
    // if char_itr.skip_while(|&ch| ch == ' ').count() != 0 {
    //   return Err("".to_string());
    // }
  }
  return result;
}

struct StateHandlingResult{
  is_recursive: bool
}

impl StateHandlingResult{
  fn new(is_recursive: bool) -> StateHandlingResult{
    return StateHandlingResult{
      is_recursive: is_recursive
    }
  }
}

fn state_handler(
  state: &mut State,
  current_char: char,
  is_in_quotes: &mut bool,
  current_key: &mut String,
  current_value: &mut Option<Json>,
  json: &mut JsonObject,
  char_itr: &mut Peekable<Chars>,
) -> Result<StateHandlingResult, String> {
  match state {
    State::Start => {
      if current_char == '{' {
        *state = State::Key;
        return Ok(StateHandlingResult::new(false));
      }
      return Err(format!("Unexpected char, {}", current_char));
    }
    State::Key => {
      if *is_in_quotes {
        if current_char == '"' {
          *is_in_quotes = false;
          return Ok(StateHandlingResult::new(false));
        }
        current_key.push(current_char);
        return Ok(StateHandlingResult::new(false));
      }
      if current_char == '}' {
        *state = State::End;
        return Ok(StateHandlingResult::new(false));
      }
      if current_char == ':' {
        *state = State::Value;
        return Ok(StateHandlingResult::new(false));
      }
      if current_char == '"' {
        *is_in_quotes = true;
        return Ok(StateHandlingResult::new(false));
      }
      return Err(format!("Unexpected char, {}", current_char));
    }
    State::Value => {
      if *is_in_quotes {
        if current_char == '"' {
          *is_in_quotes = false;
          // println!("{} : {:?}", current_key, current_value);
          json.map.insert(current_key.clone(), current_value.as_mut().unwrap().clone());
          *current_value = None;
          *current_key = String::new();
          *state = State::NextOrEnd;
          return Ok(StateHandlingResult::new(false));
        }

        if let Some(Json::String(ref mut current_value_string)) = current_value {
          current_value_string.push(current_char);
          return Ok(StateHandlingResult::new(false));
        }
        return Err("".to_string());
      }
      if current_char == '{' {
        let result = parser_recur(char_itr);
        // println!("result {:?}", result);
        if result.is_err() {
          return Err(result.err().unwrap());
        }
        *current_value = Some(Json::Object(Box::new(result.unwrap())));
        json.map.insert(current_key.clone(), current_value.as_ref().unwrap().clone());
        *state = State::NextOrEnd;
        return Ok(StateHandlingResult::new(true));
      }
      if current_char == '"' {
        *is_in_quotes = true;
        *current_value = Some(Json::String(Box::new(String::new())));
        return Ok(StateHandlingResult::new(false));
      }
      if current_char == ',' {
        *state = State::NextOrEnd;
        return Ok(StateHandlingResult::new(false));
      }
    }
    State::NextOrEnd => {
      if current_char == '"' {
        *is_in_quotes = true;
        *state = State::Key;
        return Ok(StateHandlingResult::new(false));
      }
      if current_char == '}' {
        *state = State::End;
      }
    }
    State::End => {
      // return Err(format!("Unexpected char, {}", current_char));
        return Ok(StateHandlingResult::new(false));
    }
  }
  return Ok(StateHandlingResult::new(false));
}

fn parser_recur(char_itr: &mut Peekable<Chars>) -> Result<JsonObject, String> {
  let mut is_in_quotes = false;
  let mut state = State::Start;

  let mut json: JsonObject = JsonObject {
    map: HashMap::new(),
  };

  let mut current_key: String = String::new();
  let mut current_value: Option<Json> = None;

  println!("start parsing.. {:?} {:?}",char_itr.peek(),state);

  while let Some(&current_char) = char_itr.peek() {
    if !is_in_quotes && current_char == ' ' {
      continue;
    }
    println!("state: {:?} cur char: {}", state, current_char);
    let result = state_handler(&mut state, current_char, &mut is_in_quotes, &mut current_key, &mut current_value, &mut json, char_itr);
    
    match result{
      Ok(state_handling_result) => {
        if !state_handling_result.is_recursive{
          char_itr.next();
        }
      },
      Err(err) => return Err(err)
    }
    if matches!(state,State::End){
      return Ok(json);
    }
  }
  if is_in_quotes {
    return Err("Expected \"".to_string());
  }
  if !matches!(state, State::End) {
    println!("{:?}",state);
    return Err("Unexpected end of JSON".to_string());
  }
  return Ok(json);
}
fn main() {
  // let json_string = "{\"key naja\":\"value naja\",\"key2\":\"value 2\"}";

  let json_string = r#"{"key":{}}"#;

  // let json_string = r#"{}"#;

  println!("{:?}", parser(json_string));
}
