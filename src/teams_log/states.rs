// use std::sync::atomic::AtomicBool;

// enum TSAvailability {
//     Available,
//     Busy,
//     Away,
//     BeRightBack,
//     DoNotDisturb,
//     Offline,
//     Focusing,
//     Presenting, // already in WS? Same as sharing?
//     InAMeeting, // already in WS??
//     OnThePhone, // already in WS??
// }
//
// pub struct TeamsLogStates {
//     pub availability: AtomicBool,
// }

// impl HelloWorld {
//     fn as_str(&self) -> &'static str {
//         match self {
//             HelloWorld::Hello => "Hello",
//             HelloWorld::World => "World"
//         }
//     }
// }

// If ($TeamsStatus -like "*, availability: $($_.value.keys[0])}" -or `
//      $TeamsStatus -like "*Navigation starting: about:blank?entityType=$($_.key)*") {
//  $Status = $($_.value.values[0])
//  If ($Activity -eq $taInACall -And $Status -eq $tsDoNotDisturb) {
//      $Status = $tsPresenting
//      } ElseIf ($Activity -eq $taInACall) {
//          $Status = $tsInAMeeting
//      }
// }
