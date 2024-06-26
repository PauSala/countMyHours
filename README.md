
# Count My Hours

```
 _____________________________________________________________________________________
|                                                                                      |
|   Count My Hours! (cmh)                                                              |
|                                                                                      |
|   - A simple CLI to manage your daily worktime.                                      |
|   - Add your total hours every day to keep track of your worktime.                   |
|   - If you have worked more or less than 8 hours, adds or subtracts the difference.  |
|   - You can also distribute your debt/surplus over given days.                       |
|                                                                                      |
 -------------------------------------------------------------------------------------


Usage: cmh [OPTIONS]

Options:
-a, --add <[hours]:[minutes]>  Adds [hours]:[minutes] to your daily worktime
-c, --count <[t|p]>            counts the total(t) or pending(p) hours for this month
-b, --balance                  Lists your current debt/surplus of worktime
-s, --summarize                Summarizes current status
-d, --distribute [<number>]    Distributes your current debt/surplus of worktime over given days, defaults to 5 days
-u, --undo                     Undo last addition of time, cannot be used with other flags
-i, --init-balance             Sets current balance to zero
-r, --raw                      Get raw results, not prettified
-h, --help                     Print help (see more with '--help')
-V, --version                  Print version
```

## Configuration
A default configuration is provided.
You can change the colors and the default daily hours in the configuration file.
- Create a configuration file in your home folder:  `~/.cmh.config.json`
- The configuration is a JSON object with the following properties:

```json
{
  "colors": {
    "primary": "#ECE2D0",
    "secondary": "#B79CED",
    "surplus": "#FB67EA",
    "deficit": "#F06684",
    "error": "#00FF00"
  },
  "schedule": {
    "daily_hours": "08:00",
  },
  "list_icon": "-",
  "summary_icon": "⏀"
}
```
