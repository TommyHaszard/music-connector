# We are all inside the (Music) Circle

A website for my friends to put in their top music picks for the last year and see how they connect to others.
Core repository cloned from my hottest-100 app.
Frontend was created with influence from Claude (I ain't got time to deal with JS).

## Features

- **Simple Auth** - Authentication is just a username - security is deliberately low as this is not a production application.
- **Song Search** - Search Spotify's catalog and save your top picks
- **Connections Visualization** - See your music taste connections with other users in an interactive graph
- **Compatibility Scores** - Discover who shares your music taste based on overlapping songs and artists

## Tech Stack

- **Backend**: Rust with [Rocket](https://rocket.rs/) web framework
- **Database**: PostgreSQL with [SQLx](https://github.com/launchbadge/sqlx)
- **Frontend**: Vanilla HTML/CSS/JavaScript
- **API**: Spotify Web API (client credentials flow for search)
- **Deployment**: [Fly.io](https://fly.io)

## Setup

### 1. Clone the repository

```bash
git clone <repo-url>
cd music-connector
```

### 2. Set up the database

I used Docker to run the postgres container. 

### 3. Configure environment variables

Create a `.env` file in the project root:

```env
DATABASE_URL=postgres://username:password@localhost/db
SPOTIFY_CLIENT=your_spotify_client_id
SPOTIFY_SECRET=your_spotify_client_secret
STATIC_DIR=static
ROCKET_SECRET_KEY=your_secret_key_here
```

To generate a Rocket secret key:
```bash
openssl rand -base64 32
```

### 4. Get Spotify API credentials

1. Go to [Spotify Developer Dashboard](https://developer.spotify.com/dashboard)
2. Create a new app
3. Copy the Client ID and Client Secret to your `.env` file
4. The application will use the Client Credential Flow to return a token for the user to give them access to search for songs. 

### 5. Run the app

```bash
cargo run
```

The app will be available at `http://localhost:8080`

## Project Structure

```
├── src/
│   ├── main.rs           # Rocket app setup and routes
│   └── api/
│       ├── mod.rs        # Module exports
│       ├── auth_api.rs   # Login/signup/logout endpoints
│       ├── db.rs         # Database queries
│       ├── external_api.rs # Spotify API integration
│       ├── internal_api.rs # Page routes and internal APIs
│       └── types.rs      # Request/response types
├── static/
│   ├── index.html        # Main song selection page
│   ├── login.html        # Login page
│   ├── signup.html       # Signup page
│   ├── connector.html    # Connections visualization
│   ├── design-system.css # Shared styles
│   └── script.js         # Main page JavaScript
├── db/
│   └── init.sql          # Database schema
├── Cargo.toml            # Rust dependencies
├── fly.toml              # Fly.io deployment config
└── Dockerfile            # Container build
```

## API Endpoints

### Auth
- `POST /api/login` - Login with username
- `POST /api/signup` - Create account with username, first name, last name
- `POST /api/logout` - Logout

### Songs
- `GET /search-songs?track=<query>&rank=<rank>` - Search Spotify
- `POST /songs` - Save user's song rankings
- `GET /songs` - Get user's saved songs

### Connections
- `GET /music-taste-user` - Get current user's connections with compatibility scores

## Deployment

The app is configured for Fly.io deployment:

```bash
fly launch
fly secrets set DATABASE_URL="..." SPOTIFY_CLIENT="..." SPOTIFY_SECRET="..." ROCKET_SECRET_KEY="..."
fly deploy
```

## How Connections Work

The compatibility score between users is calculated based on:
- **Overlapping songs** - Songs both users have ranked
- **Overlapping artists** - Different songs by the same artist
- **Rank similarity** - How close the rankings are for shared items

Connection strength is visualized with colors:
- Green: Strong connection (score >= 20)
- Yellow: Medium connection (score 10-19)
- Red: Weak connection (score < 10)
