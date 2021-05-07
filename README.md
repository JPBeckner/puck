# Puck
Command Line Interface application in Rust to "Punch the Clock".

> PUnch the cloCK -> PUCK

## Commands

### Punch
Register a new hour that you are arriving or leaving.
You must inform if that hour is to in or to out.

* `io`: `in` or `out`
* `hour`: Must folow this format: "hour-minutes". e.g. "12-30". If not specified, takes atual hour.
* `user`: Optional argument that specify a user, other then default, to register his hour.

### Balance
Check your hours balance.

* `when`: Optional argument that inform a specific date to check, following this format: "day-month-year". e.g. "30-12-2000". If not informed, it will check actual date.
* `user`: Optional argument that specify a user, other then default, to check his hour.

### User
Configure a user name, who gonna "Punch the Clock"

* `new`: User name, to register a new user.
* `default`: Define the default user.
