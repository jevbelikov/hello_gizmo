
fn get_greeting(name: &str) -> &str {
  if name.to_lowercase() == "gizmo" {
	 return "Hello Boss!";
  }
  
  "Argh, another drone ..."
}

#[test]
fn test_get_greeting() {
  assert_eq!(get_greeting("gizmo"), "Hello Boss!");
  assert_eq!(get_greeting("Gizmo"), "Hello Boss!");
  assert_eq!(get_greeting(""),      "Aggh, another drone ...");
  assert_eq!(get_greeting("Sulik"), "Aggh, another drone ...");
}

fn main() {
    println!("{}", get_greeting("Gizmo"));
    println!("{}", get_greeting("Sulik"));
}
