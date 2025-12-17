/**
 * @file api.js
 * @description Struktura API Client v2.3 - Command Center
 * Mission briefing: Central coordination hub for all operations
 *
 * Architecture: Modular deployment with strategic separation of concerns
 */

import { ApiError } from "./models.js";
import { createAuthModule } from "./auth.js";
import { createUserModule } from "./user.js";
import { createCalculusModule } from "./calculus.js";

// =============================================================================
// CORE CLIENT CLASS
// =============================================================================

class StrukturaClient {
  constructor() {
    this.baseUrl = "/api/v1";
    this.csrfToken = null;
    this.isAuthenticated = false;

    // Internal intelligence cache
    this.cache = {
      beginnerCatalogue: null,
      engineerCatalogue: null,
      contractorCatalogue: null,
    };

    // Initialize operational modules
    this.auth = createAuthModule(this.#request.bind(this), this);
    this.user = createUserModule(this.#request.bind(this));
    this.calculus = createCalculusModule(this.#request.bind(this), this.cache);
  }

  /**
   * Internal request handler - the diplomatic courier
   * Handles all communications with headquarters
   */
  async #request(endpoint, options = {}) {
    const { method = "GET", body, headers = {}, ...customConfig } = options;

    const config = {
      method,
      headers: {
        "Content-Type": "application/json",
        Accept: "application/json",
        ...headers,
      },
      credentials: "same-origin",
      ...customConfig,
    };

    // Auto-inject CSRF Token for mutating operations
    if (this.csrfToken && ["POST", "PUT", "PATCH", "DELETE"].includes(method)) {
      config.headers["X-CSRF-Token"] = this.csrfToken;
    }

    if (body) {
      config.body = JSON.stringify(body);
    }

    try {
      const response = await fetch(`${this.baseUrl}${endpoint}`, config);

      // Update CSRF if rotated by server - key rotation protocol
      const newCsrf = response.headers.get("X-CSRF-Token");
      if (newCsrf) this.csrfToken = newCsrf;

      // Handle Empty Responses (204 No Content)
      if (response.status === 204) return null;

      // Parse JSON intelligence
      const data = await response.json().catch(() => null);

      // Handle operational failures
      if (!response.ok) {
        const errorMsg = data?.error || data?.message || response.statusText;
        throw new ApiError(errorMsg, response.status, data);
      }

      return data;
    } catch (error) {
      if (error instanceof ApiError) throw error;
      throw new ApiError(error.message || "Network request failed", 0);
    }
  }

  /**
   * Initialize the client - mission preparation
   * Establishes secure communications and preloads intelligence
   */
  async init() {
    try {
      // Acquire CSRF Token - establish secure channel
      const data = await this.#request("/auth/csrf");
      this.csrfToken = data?.csrf_token || null;

      // Verify authentication status
      try {
        await this.user.me();
        this.isAuthenticated = true;
      } catch (e) {
        this.isAuthenticated = false;
      }

      // Preload operational catalogues for rapid deployment
      this.calculus.preloadCatalogues();

      return { ready: true, authenticated: this.isAuthenticated };
    } catch (error) {
      console.error("Struktura API Init Failed:", error);
      return { ready: false, error };
    }
  }
}

// =============================================================================
// SINGLETON EXPORT
// =============================================================================

/**
 * Primary operational instance
 * For most missions, this is your command center
 */
export const api = new StrukturaClient();

/**
 * Export the class for multiple instance scenarios
 * When you need separate operational theaters
 */
export { StrukturaClient };
