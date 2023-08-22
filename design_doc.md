## Product spec
* Print out what workouts to do and a countdown timer
* [Stretch] Narrate workouts out loud
* [Stretch] Read exercises from a db
* [Stretch] Log workouts to db (flexible choice of db)
* [Stretch] Play/pause music



## How
* Load workouts from json file
  * get git set up for personal account
  * parse the JSON into some struct type
* Load config from json file
  * could also allow clap to take a subcommand with params to generate your own config
  * could also allow naming of configs so that you emit an error message telling you what named configs are available to select
* Take params from command line and put in struct
* Combine params and json data to create a workout
* Play the workout