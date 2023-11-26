# SSD_Nat_Flowers_User_Info
School project for Secure Software Development ethical hack. There are 2 section to this project.
1. bash scripts inside of the scripts/ folder that use cURL to send various requests to the server. These are completely independant of the other program, and are what I used for testing access to API endpoints.
2. A program that gets all userIds from the /reviews endpoint, then takes those userIds and constructs a get request to the /profile/{userId} endpoint for each userId. This shows all user info associated with that user.

**This is not a production website, and this user information is not real. This is for testing purposes only.**

# How to use the program to see user data
This code can be ran on any computer using the following instructions
1. Install Rust and Cargo. This can be done by following the instructions [Here.](https://www.rust-lang.org/tools/install)
2. Clone this repository to you local machine.
3. Navigate to where the repository has been cloned to and run the command `cargo run`

### How are we getting the user data?
This program executes the following instructions.
1. Make a get request to the /review endpoint. This shows all reviews in the database. These reviews contain the userId.
2. De-serialize json data from reviews and get a list of unique userIds.
3. Make a get request to /profile/{userId} for each unique userId. This request will return all user information about the user with that userId.

#### **A sample of the user info that can be collected can be seen in the `info` file within this git repo.**
