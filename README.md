# Lockermatch

## Background

Lockermatch is a tool for matching students to lockers, initially developed for California High School in the SF Bay Area. This project started in 2022 with Adam Blumenfeld (CEO, CSX Labs) and Aswath Subramanian (former Software Engineer, CSX Labs | current CSE @ Cal Poly SLO) were trying to find projects to complete with their school. We worked with many people, including most notably former Assistant Principal, Mr. Osborn (now promoted) and current Principal, Mr. Ball. We also coordinated with Ms. Hilton (CTO) at the SRVUSD district office and her staff.

We originally inherited a codebase from a previous student group which wasn't safe for student data. We started from scratch and built a new system with an Actix-web backend and a Svelte frontend, with custom assignment and matching algorithms, and a hand-written Rust connector to Google Sheets to store and retrieve student data and lockers (as requested by the school).

We ultimately got locked up in approvals for the project as we licensed it to Adam's company, [CSX Labs](https://csxlabs.org), and we encountered some resistance from the school district's IT department around handling of student data and the hosting of the application as a third-party student-run platform.

Now it's 2025 and we're building a new version of the application that is open source and can be self-hosted. It's meant to be secure and easy to deploy and maintain by future students. Also, as most of what we produce is proprietary, we thought it would be a good project to showcase our capabilities as individuals and as a company.

## Architecture

This project premise isn't as complex as other stuff we've built, so we can have a little fun with the tech stack. Specifically, we are going to try out Axum and see if we can bundle React with SSR within our Rust application to make it full stack and self-contained.

We also will ensure that the application is SOPIPA compliant, FERPA compliant, and we can handle the student data securely.

### Backend

- Rust
- Axum: Opinionated, supported by Tokio, and safer than Actix-web.
- Redis: We are experimenting with this as an application database due to it's speed and ease of use. However, as it's a KV store, we'll need to think about encryption, RBAC, and our architecture.
- Integration with Active Directory: we'll need to use this to authenticate students, staff, and administrators with an OTP based system.
- SMTP: we'll create an email service to send emails to students, staff, and administrators.

### Frontend

- React with React Router v7: Tired of NextJS.
- Vite: Webpack is mid.
- Tailwind: We'll need strict component-driven development to make sure our code doesn't look like shit, however I love tailwind.

### Deployment

- Docker: containerization.
- Kubernetes: orchestration.
- Github Actions: CI/CD.
- On-premise: district-owned servers.

### Security

- SOPIPA & FERPA: Record management, we'll encrypt with FIPS-compliant ciphers, and we'll use RBAC to manage access to the data.
- Authentication: OTP based system with Active Directory.
- Authorization: RBAC with JWT tokens.
