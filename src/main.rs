#[derive(Debug)]
enum State{
  Start,
  Key,
  Value,
  NextOrEnd,
  End
}

enum Json{
  String(Box<String>),
  Json(Box<Json>),
  Array(Vec<Box<Json>>)
}

fn parser(json_string: &str) -> Result<(),String>{
  let mut is_in_quotes = false;
  let mut char_itr = json_string.chars();
  let mut state = State::Start;

  let mut current_key: String = String::new();
  let mut current_value: String = String::new();
  while let Some(current_char) = char_itr.next(){
    if !is_in_quotes && current_char==' '{
      continue;
    }
    println!("cur state: {:?} cur char: {}",state,current_char);
    match state{
      State::Start => {
        if current_char=='{'{
          state=State::Key;
          continue;
        }
        return Err(format!("Unexpected char, {}",current_char));
      },
      State::Key => {
        if is_in_quotes{
          if current_char=='"'{          
            is_in_quotes=false;
            continue;
          }
          current_key.push(current_char);
          continue;
        }
        if current_char==':'{
          state=State::Value;
          continue;
        }
        if current_char=='"'{
          is_in_quotes=true;
          continue;
        }
        return Err(format!("Unexpected char, {}",current_char));
      },
      State::Value => {
        if is_in_quotes{
          if current_char=='"'{          
            is_in_quotes=false;
            println!("{} : {}",current_key,current_value);
            current_key=String::new();
            current_value=String::new();
            state=State::NextOrEnd;
            continue;
          }
          current_value.push(current_char);
          continue;
        }
        if current_char=='"'{
          is_in_quotes=true;
          continue;
        }
        if current_char==','{
          state=State::NextOrEnd;
          continue;
        }
      },
      State::NextOrEnd => {
        if current_char=='"'{
          is_in_quotes=true;
          state=State::Key;
          continue;
        }
        if current_char=='}'{
          state=State::End;
        }
      },
      State::End => {
        return Err(format!("Unexpected char, {}",current_char));
      }
    }
  }
  if is_in_quotes{
    return Err("Expected \"".to_string());
  }
  if !matches!(state,State::End){
    return Err("Unexpected end of JSON".to_string());
  }
  return Ok(());
}
fn main() {
    println!("{:?}",parser("{\"key naja\":\"value naja\",\"key2\":\"value 2\"}"));
}
