# 🦀 *IronCrab — because crabs don't skip leg day.*

<img width="1024" height="572" alt="image" src="https://github.com/user-attachments/assets/50301763-b937-4d65-b3e2-f5d41bd50ebb" />


> *Because your compiler has better form than your spotter.*



A command-line workout program builder written in Rust. Tell IronCrab your goals, your available equipment, and how many days a week you can train — it builds you an optimized program. No subscription, no algorithm you can't see, no premium tier. Just a fast Rust binary and a local database.



---



## What It Does



- Fetches a full exercise library from the [API Ninjas Exercises API](https://api-ninjas.com/api/exercises) and stores it locally

- Lets you query exercises by muscle group, equipment, or difficulty

- *(Coming soon)* Generates a weekly training plan based on your constraints

- *(Coming soon)* Tracks your sessions and progression over time



---



## Project Status



| Phase | Description                                            | Status         |
| ----- | ------------------------------------------------------ | -------------- |
| 1     | Models — Exercise, MuscleGroup, Equipment, Difficulty  | ✅ Done         |
| 2     | Scraper — fetch from API Ninjas, parse into models     | ✅ Done         |
| 3     | Database — persist exercises to SQLite                 | 🔨 In progress |
| 4     | CLI — `ironcrab sync`, `ironcrab plan`, `ironcrab log` | 🔲 Planned     |
| 5     | Planner — constraint-based weekly program generator    | 🔲 Planned     |
| 6     | Progress tracking — log sessions, track over time      | 🔲 Planned     |


---



## Getting Started



### Prerequisites



- [Rust](https://rustup.rs/) (stable, 2021 edition or later)

- An API key from [API Ninjas](https://api-ninjas.com/) — free tier is enough



### Installation



```bash

git clone https://github.com/yourusername/iron-crab

cd iron-crab

```



### Configuration



Create a `.env` file in the project root:



```bash

echo "NINJAS_API_KEY=your_api_key_here" > .env

```

### Run



```bash

cargo run

```

---



## Data Source



Exercises are fetched from the **[API Ninjas Exercises API](https://api-ninjas.com/api/exercises)**.

---


## Roadmap



- [ ] SQLite persistence with `rusqlite`

- [ ] `ironcrab sync` — fetch and store full exercise library

- [ ] `ironcrab plan` — generate a weekly program from constraints

- [ ] `ironcrab log` — record completed sessions

- [ ] `ironcrab history` — view past sessions

- [ ] `ironcrab progress` — track progression per muscle group

- [ ] Constraint system: equipment filter, injury avoidance, time-per-session budget

- [ ] Optimizer — score and rank generated plans

---
