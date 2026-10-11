import { CreateIncidentInput, IncidentRecord } from '../types/incident';

export class PranaVayuApiClient {
  private endpoint: string;

  constructor(endpoint: string = '') {
    this.endpoint = endpoint.trim().replace(/\/+$/, '');
  }

  public setEndpoint(endpoint: string): void {
    this.endpoint = endpoint.trim().replace(/\/+$/, '');
  }

  public getEndpoint(): string {
    return this.endpoint;
  }

  public isConfigured(): boolean {
    return this.endpoint.length > 0;
  }

  /**
   * Fetches latest incidents from the backend API.
   */
  public async fetchIncidents(): Promise<IncidentRecord[]> {
    if (!this.isConfigured()) {
      throw new Error('API endpoint is not configured. Operating in local mode.');
    }

    const response = await fetch(`${this.endpoint}/incidents`, {
      method: 'GET',
      headers: {
        'Accept': 'application/json',
      },
    });

    if (!response.ok) {
      const errorText = await response.text();
      throw new Error(`HTTP ${response.status}: ${errorText || response.statusText}`);
    }

    const data = await response.json();
    return Array.isArray(data) ? data : data.incidents || [];
  }

  /**
   * Dispatches a new incident record to the backend Lambda API.
   */
  public async createIncident(input: CreateIncidentInput): Promise<IncidentRecord> {
    if (!this.isConfigured()) {
      throw new Error('API endpoint is not configured.');
    }

    const response = await fetch(`${this.endpoint}/incidents`, {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        'Accept': 'application/json',
      },
      body: JSON.stringify(input),
    });

    if (!response.ok) {
      const errorText = await response.text();
      throw new Error(`HTTP ${response.status}: ${errorText || response.statusText}`);
    }

    return await response.json();
  }
}
