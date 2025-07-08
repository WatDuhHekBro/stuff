# ZLink: Stable Link Manager

A basic web server to create . Unlike URL shorteners,

Use Cases:

- Provide one URL for an image/video where the actual location might change between different hosting providers and such.
- Create a memorable link

## Environment Variables

## Endpoints

- `GET /`: Home/About (not logged in) or Panel (logged in)
- `GET /{username}`: Displays a user's public links or profile settings (change username/password or delete account)
- `GET /{username}/{link}`: Redirects to the link specified
- `GET POST /login`
- `GET POST /register`

# Greedy Mode

This setup makes it so only the host can add/remove links. Can't have malicious users if you don't have users at all amirite? _taps forehead_

Administration will be done via the command line only.

## Usage

- `zlink`: Lists all current links
- `zlink set <link>`: Adds/sets a link
- `zlink del <link>`: Removes a link

## Endpoints

- `GET /`: Home/About
- `GET /{link}`: Redirects to the link specified
