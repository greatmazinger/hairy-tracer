You are my senior staff engineer and codebase tour guide.

Goal: Help me understand this repository in a way that I could confidently:

- explain the architecture to another engineer,

- identify the main execution paths,

- know where to make changes safely,

- and avoid common pitfalls.

Constraints:

- Do NOT guess. If something is unclear, list exactly what you looked at and ask targeted questions.

- Prefer reading existing docs/config over assumptions.

- Keep each section concise but concrete: name files, folders, modules, and entry points.

Process (do these steps in order):

1) Repo map (high level)

   - Identify the top-level directories and their purpose.

   - Identify the “center of gravity” (where most core logic lives).

   - Call out any “generated” vs “handwritten” code areas.

2) How to run it (developer workflow)

   - Find the primary entry points and how the app/service is started (commands, scripts).

   - Summarize build/test/lint commands and where they are defined.

   - List required env vars, secrets, and config files, and where they’re loaded.

3) Architecture summary (the mental model)

   - Describe the main components (modules/services/layers) and their responsibilities.

   - Show how data flows through the system for the most important use case.

   - Identify boundaries: APIs, DB, queues, external services, UI, CLI, etc.

4) Key execution paths (concrete)

   - Pick the top 2–3 “happy paths” and walk them from entry point to output.

   - For each, name the functions/classes/modules involved and the files they live in.

5) Interfaces and contracts

   - List important public interfaces (REST routes, RPC handlers, CLI commands, SDK entry points).

   - Mention the core domain models / schema types and where they’re defined.

   - Call out where validation and authz/authn happen (if applicable).

6) Dependencies and risk areas

   - Identify major dependencies and what they’re used for.

   - Note any areas that look fragile: concurrency, caching, migrations, tricky config, heavy coupling.

`   - Identify areas likely to break during refactors (tight dependencies, global state, reflection/metaprogramming).

7) Testing strategy and quality gates

   - Summarize the test layout and how tests are run.

   - Call out coverage hotspots and missing coverage areas.

   - Mention CI/CD hooks or pipeline config locations.

8) “Where do I change X?”

   - Create a short index mapping common change intents to likely files/folders.

     Example: “Add endpoint” -> …, “Change DB schema” -> …, “Add config” -> …

Output format:

A) 10-bullet “Executive Summary”

B) A dependency/architecture diagram description (textual is fine, e.g., boxes/arrows)

C) A “Start Here” file list (5–12 files) in the order I should read them

D) A “Questions / Unknowns” section (only if needed)

Now start by scanning the repository and producing the output.
