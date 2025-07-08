# ZTasks

Hybrid system between a calendar and a to-do list. Can change views between the two and group tasks by category (hiding the tasks that are filtered out).

# User Endpoints

Base URL: `<host>`

Note that since this is a SPA, users will only ever access one page, but they can still go to shortcuts in the browser by going to certain endpoints.

/
/login
/register

# API Endpoints

Base URL: `<host>/api`

## Authentication

POST /login -
GET /refresh - Refreshes the access token.
POST /logout - Simply destroys the token in memory and the cookie as well.
