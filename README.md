# PomAPI

A simple Axum API server for timers

## Features

- `/quote` currently gives a single hard-coded motivational quote

- `/timer/{duration in minutes}` creates a new timer and returns its:
  - unique ID
  - start time in UTC
  - end time in UTC

- `/status/{timer id}` returns the remaining number of seconds, number of minutes rounded down, and number of hours rounded down. If the timer has already expired, these numbers are negative. 
