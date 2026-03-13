# 🦀 IronCrab



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



| Phase | Description | Status |

|---|---|---|

| 1 | Models — Exercise, MuscleGroup, Equipment, Difficulty | ✅ Done |

| 2 | Scraper — fetch from API Ninjas, parse into models | ✅ Done |

| 3 | Database — persist exercises to SQLite | 🔨 In progress |

| 4 | CLI — `ironcrab sync`, `ironcrab plan`, `ironcrab log` | 🔲 Planned |

| 5 | Planner — constraint-based weekly program generator | 🔲 Planned |

| 6 | Progress tracking — log sessions, track over time | 🔲 Planned |



---



## Project Structure



```

iron-crab/

├── src/

│   ├── main.rs              ← entry point

│   ├── models/              ← data types (Exercise, Plan, WorkoutDay...)

│   │   ├── mod.rs

│   │   ├── exercise.rs

│   │   ├── muscle_group.rs

│   │   ├── equipment.rs

│   │   ├── difficulty.rs

│   │   ├── plan.rs

│   │   └── workout.rs

│   ├── scraper/             ← fetches data from API Ninjas

│   │   ├── mod.rs

│   │   └── api_ninjas.rs

│   ├── db/                  ← SQLite persistence (coming soon)

│   ├── planner/             ← program generation logic (coming soon)

│   ├── cli.rs               ← command definitions (coming soon)

│   └── display/             ← terminal output formatting (coming soon)

├── .env                     ← API key (never commit this)

├── .gitignore

└── Cargo.toml

```



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



> ⚠️ Never commit your `.env` file. It is already in `.gitignore`.



### Run



```bash

cargo run

```



---



## Dependencies



```toml

reqwest  = { version = "0.12", features = ["json"] }  # HTTP client

serde    = { version = "1",    features = ["derive"] } # JSON parsing

tokio    = { version = "1",    features = ["full"] }   # async runtime

anyhow   = "1"                                         # error handling

dotenvy  = "0.15"                                      # .env loading

```



---



## Data Source



Exercises are fetched from the **[API Ninjas Exercises API](https://api-ninjas.com/api/exercises)**.



Each exercise includes:



| Field | Example |

|---|---|

| `name` | `"Barbell Curl"` |

| `muscle` | `"biceps"` |

| `type` | `"strength"` |

| `difficulty` | `"intermediate"` |

| `equipments` | `["barbell"]` |

| `instructions` | Full text instructions |

| `safety_info` | Safety guidance |



16 muscle groups covered: biceps, triceps, chest, lats, middle back, lower back, shoulders, quadriceps, hamstrings, calves, glutes, abdominals, forearms, traps, abductors, adductors.



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



## Why Rust?



IronCrab is a learning project. The goal is to build something genuinely useful while exploring Rust's type system, error handling, async runtime, and CLI tooling — all in one project that touches HTTP, serialization, databases, and algorithms.



---



## License



MIT — use it, fork it, build on it.



---



*IronCrab — because crabs don't skip leg day.*
