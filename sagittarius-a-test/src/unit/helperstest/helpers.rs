
// #[cfg(test)]

// use sagittarius_a_model::usermodel::UserId;
// use sagittarius_a_utils::helpers::access_controll_helper::validate_token_checking_user;

// mod test_access_controll_helper {

//     #[actix_rt::test]
//     async fn test_validate_token_checking_user() {
//         let usr = super::UserId {
//             id: String::from("9fde1fe5-a1c3-4ba6-9165-521bb696b54b"),
//         };

//         let token = "eyJhbGciOiJIUzM4NCJ9.eyJkYXRhX2V4cCI6IklORiIsImlkIjoiZS9SeTVsR0F4ZFR5YzJBV0lzTUFoM1d1SU9JTHNTODVBMlJSdmNDR25HTlpvN2VLYmZSR251aGg0WkROVEdaNiIsInBlcmZpbCI6ImJUVCtwQ3RnOWRYM0M4c2N4SFBjakE9PSIsInVzZXJfbmFtZSI6Im44blFuQ1pYUmpZRWJGa1ZiUG1SSnc9PSJ9.-qZJc76r7z1bS4J33YIntxJl-nsjrgN46XdvZo66ou-eK0XV2OHr8KUXFbu72iTe";

//         let result = super::validate_token_checking_user(&token.to_string(), &usr).await;

//         assert_eq!(result.unwrap(), true);
//     }



//     #[actix_rt::test]
//     #[should_panic(expected = "O token deve ser inválido!")]
//     async fn test_validate_token_checking_user_error() {
//         let usr = super::UserId {
//             id: String::from("9fde1fe5-a1c3-4ba6-9165-521bb696b54b"),
//         };

//         let token = "eyJhbGciOiJIUzM4NCJ9.eyJkYXRhX2V4cCI6IklORiIsImlkIjoiZS9SeTVsR0F4ZFR5YzJBV0lzTUFoM1d1SU9JTHNTODVBMlJSdmNDR25HTlpvN2VLYmZSR251aGg0WkROVEdaNiIsInBlcmZpbCI6ImJUVCtwQ3RnOWRYM0M4c2N4SFBjakE9PSIsInVzZXJfbmFtZSI6Im44blFuQ1pYUmpZRWJGa1ZiUG1SSnc9PSJ9.-qZJc76r7z1bS4J33YIntxJl-nsjrgN46XdvZo66ou-eK0XV2OHr8KUXFbu72iTe";

//         let result = super::validate_token_checking_user(&token.to_string(), &usr).await;

//         assert!(result.unwrap());
//     }

// }
