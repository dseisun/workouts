## Product spec
* Print out what workouts to do and a countdown timer
* [Stretch] Narrate workouts out loud
* [Stretch] Read exercises from a db
* [Stretch] Log workouts to db (flexible choice of db)
* [Stretch] Play/pause music
* Ability to print out or copy/paste the workout without narration



## How
* Load config from json file
  * Take param for config path, otherwise use default path
  * Deserialize into config object
  * could also allow clap to take a subcommand with params to generate your own config
  * could also allow naming of configs so that you emit an error message telling you what named configs are available to select


* Load workouts from json file
  * get git set up for personal account -> COMPLETE
  * parse the JSON into some struct type
* Take params from command line and put in struct
* Play the workout

## Config logic
### Duplicates
* We could either do a pick w/replace (meaning a given workout could be in a run several times depending on chance), or a pick without replacement, and start the list from scratch once you've picked all the exercises
### Manual insert
* Ability to make sure a given exercise is in the workout
* This exercise should be added first, and allow the exercise type to go over the time allotted if needed

### Whitelist
* Constrains the list of workouts to a specific list of exercises


THOUGHT: Should exercises be enums? Or should they just be referenced by id?