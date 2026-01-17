# AI Collaboration Instructions

These instructions define the preferred architectural patterns and working style for our collaboration. Follow these principles for every task to ensure high-quality, production-ready results.

## 🏗️ Architectural Philosophy: Domain-Driven Design (DDD)

Always structure the codebase using a strict 4-layer separation of concerns. This ensures maintainability and allows for swapping presenters (e.g., CLI to Web) or infrastructure (e.g., Database to API) without touching core logic.

1.  **Domain Layer**: 
    *   **Core Entities & Value Objects**: Pure business models.
    *   **Repository Interfaces**: Define *what* data is needed, not *how* it is fetched.
    *   **Domain Errors**: Centralized, semantic error types.
    *   *Constraint*: Zero dependencies on external frameworks or infrastructure.

2.  **Application Layer**:
    *   **Use Cases**: Orchestrate flow between Domain and Infrastructure.
    *   **Input/Output**: Handle application-specific logic (e.g., filtering, bulk operations).
    *   *Constraint*: Depends only on the Domain layer.

3.  **Infrastructure Layer**:
    *   **Implementations**: Concrete logic for API clients, database access, config management, etc.
    *   *Constraint*: Implements interfaces defined in the Domain.

4.  **Presenter Layer**:
    *   **User Interfaces**: CLI, TUI, REST Controllers, or Web UIs.
    *   **User Feedback**: Handle formatting, colors, and interactive prompts.
    *   *Constraint*: Delegates all business logic to the Application layer.

## 🧪 Quality & Verification (The "Zero-Bugs" Pattern)

*   **Automated Testing is Mandatory**: 
    *   New features must include unit tests for Use Cases (using Mocks/Stubs).
    *   Presenter logic (like CLI commands) must include integration tests (e.g., verifying help output or exit codes).
*   **Proactive Error Fixing**: If a command or build fails, don't just report it—investigate the source (e.g., reading library source code or API docs), implement a fix, and verify it immediately.
*   **Zero Placeholders**: Never leave `TODO` comments or incomplete functions. Provide fully working implementations.

## 🎨 Aesthetic & UX Excellence

*   **Premium Presentation**: Even in CLI tools, prioritize clarity. Use tables, colors, and clear status messages to guide the user.
*   **Detailed Feedback**: Operations (especially bulk/destructive ones) must provide a detailed summary of what was changed (e.g., listing specific deleted items rather than just a count).
*   **Semantic Feedback**: Use appropriate colors (Cyan for info, Green for success, Red for errors, Yellow for warnings).

## 🤝 Collaborative Working Pattern

*   **Autonomy with Transparency**: Be proactive in proposing and executing steps. Explain *why* a certain pattern (like DDD) is being applied.
*   **Traceability**: When editing files, provide clear descriptions of changes.
*   **Fail Fast**: If a requirement is ambiguous, ask for clarification immediately before proceeding with a large implementation.
