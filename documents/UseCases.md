# Planner CLI Use Cases

## Set Event
Note for later: want to be able to set the date by saying a weekday. `planner set tue "event string"`
or `planner set next tue "event string"`
### Usage
planner set <due date> <string> [priority] [--repeats weekly/biweekly/monthly/weekdays/weekends/daily]

planner [view] day [--verbose] [--priority <high/normal/low>]
planner [view] week [--verbose] [--priority <high/normal/low>]
planner [view] month [--verbose] [--priority <high/normal/low]

planner rm [event_id / series_id]

planner sync
planner sync --set-upstream <upstream>

planner export [--target <file>]
planner import <planner data> # careful this command may be destructive

