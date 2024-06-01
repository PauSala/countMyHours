
# Count My Hours

```bash
______________________________________________________________________________________
|                                                                                      |
|   Count My Hours! (cmh)                                                              |
|                                                                                      |
|   - A simple CLI to manage your daily worktime.                                      |
|   - Add your total hours every day to keep track of your worktime.                   |
|   - If you have worked more or less than 8 hours, adds or subtracts the difference.  |
|   - You can also distribute your debt/surplus over given days.                       |
|                                                                                      |
--------------------------------------------------------------------------------------


Usage: cmh [OPTIONS]

Options:
 -a, --add <[hours]:[minutes]>
         Adds [hours]:[minutes] to your daily worktime

 -c, --count <[t|p]>
         counts the total(t) or pending(p) hours for this month

 -b, --balance
         Lists your current debt/surplus of worktime

 -s, --summarize
         Resumes your current status

 -d, --distribute [<number>]
         Distributes your current debt/surplus of worktime over given days, defaults to 5 days

 -u, --undo
         Undo last addition of time, cannot be used with other flags

 -h, --help
         Print help (see a summary with '-h')

 -V, --version
         Print version
```

## Configuration
A default configuration is provided.
You can change the colors and the default daily hours in the configuration file.
- Create a configuration file:  `~/.cmh.config.json`
## The configuration is a JSON object with the following properties:

```json
{
  "colors": {
    "primary": "#ECE2D0",  //default color
    "secondary": "#B79CED",//main color
    "surplus": "#FB67EA", //color for surplus
    "deficit": "#F06684",   //color for deficit
    "error": "#00FF00" //color for error messages
  },
  "schedule": {
    "daily_hours": "08:00",
    "week_wd": 5 //currently not used
  },
  "list_icon": "-",
  "summary_icon": "⏀"
}
```
