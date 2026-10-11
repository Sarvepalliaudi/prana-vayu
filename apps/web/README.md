# PRANA-VAYU Web Interfaces & Contracts (`apps/web`)

This directory provides TypeScript definitions and API integration client utilities for PRANA-VAYU:

- **`src/types/incident.ts`**: TypeScript type definitions mirroring the Rust domain model (`IncidentRecord`, `Severity`, `LifecycleState`, `ResearchDossier`, `GeoCoordinate`).
- **`src/services/api.ts`**: Client integration wrapper for AWS API Gateway / serverless endpoints.

### Note on Production Web Deployment
The authoritative production application and interactive operational console is located at [`index.html`](../../index.html) in the repository root. AWS Amplify builds and deploys this frontend using [`amplify.yml`](../../amplify.yml) to generate `dist/index.html`.
