use serde::{ Deserialize, Serialize };
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, Validate, PartialEq, Eq, Clone)]
pub struct Person {
	pub uuid: String,
	#[validate(length(min = 1, message = "Name is required"))]
	pub name: String,
	#[validate(length(min = 1, message = "Title is required"))]
	pub title: String,
	#[validate(length(min = 1, message = "Level is required"))]
	pub level: String,
	#[validate(range(min = 2000, max = 9999))]
	pub compensation: i32,
	pub joined_at: String,
}

impl Person {
	pub fn new(
		uuid: String,
		name: String,
		title: String,
		level: String,
		compensation: i32,
		joined_at: String
	) -> Self {
		Self {
			uuid,
			name,
			title,
			level,
			compensation,
			joined_at,
		}
	}
}

#[derive(Debug, Validate, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct AddPersonRequest {
	#[validate(length(min = 1, message = "Name is required"))]
	pub name: String,
	#[validate(length(min = 1, message = "Title is required"))]
	pub title: String,
	#[validate(length(min = 1, message = "Level is required"))]
	pub level: String,
	#[validate(range(min = 2000, max = 9999))]
	pub compensation: i32,
}

impl AddPersonRequest {
	pub fn new(name: String, title: String, level: String, compensation: i32) -> Self {
		Self {
			name,
			title,
			level,
			compensation,
		}
	}
}
