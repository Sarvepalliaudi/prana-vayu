/**
 * PRANA-VAYU Operational Types
 * Strict typed definitions matching Rust prana-core and Python Lambda schemas.
 * 
 * Developer: SARVEPALLI AUDI SIVA BHANUVARDHAN
 * Brand: AS PHENIX
 */

export type Jurisdiction = 'India' | 'US' | 'UK' | 'EU' | 'Global';

export type PollutionSource = 
  | 'Biomass Burning'
  | 'Vehicle Idling'
  | 'Silica Dust'
  | 'Kitchen Exhaust'
  | 'Perimeter Waste Combustion';

export type Severity = 'Moderate' | 'High' | 'Severe';

export type LifecycleState = 
  | 'Reported'
  | 'Validated'
  | 'Assessed'
  | 'ResponseActive'
  | 'Documented'
  | 'Resolved'
  | 'Archived';

export interface ResearchDossier {
  particulate_profile: string;
  immediate_protocol: string[];
  long_term_engineering: string[];
  statutory_memo: string;
}

export interface IncidentRecord {
  incident_id: string;
  timestamp: string;
  jurisdiction: Jurisdiction;
  zone: string;
  source: PollutionSource;
  severity: Severity;
  notes: string;
  lat?: number | null;
  lng?: number | null;
  lifecycle_state: LifecycleState;
  analysis_provider: string;
  analysis_status: 'completed' | 'fallback' | 'failed' | 'pending';
  research_dossier?: ResearchDossier | null;
}

export interface CreateIncidentInput {
  jurisdiction: Jurisdiction;
  zone: string;
  source: PollutionSource;
  severity: Severity;
  notes: string;
  lat?: number | null;
  lng?: number | null;
}

export interface IncidentFilter {
  searchQuery?: string;
  severity?: Severity | 'ALL';
  jurisdiction?: Jurisdiction | 'ALL';
  source?: PollutionSource | 'ALL';
}
