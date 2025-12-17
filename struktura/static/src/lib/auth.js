/**
 * @file auth.js
 * @description Authentication operations - the diplomatic corps
 * Mission objective: Secure credentials management and session control
 */

export function createAuthModule(requestHandler, context) {
  return {
    /**
     * Initiate authorized access - establishing diplomatic relations
     */
    login: async (email, password) => {
      const res = await requestHandler("/auth/login", {
        method: "POST",
        body: { email, password },
      });
      context.isAuthenticated = true;
      return res;
    },

    /**
     * Recruit new operative - onboarding protocols
     */
    signup: async (email, password, name) => {
      const res = await requestHandler("/auth/signup", {
        method: "POST",
        body: { email, password, name },
      });
      context.isAuthenticated = true;
      return res;
    },

    /**
     * Terminate mission - burn after reading
     */
    logout: async () => {
      await requestHandler("/user/logout", { method: "POST" });
      context.isAuthenticated = false;
      context.csrfToken = null;

      // Re-establish public access credentials
      const data = await requestHandler("/auth/csrf");
      context.csrfToken = data?.csrf_token;
    },

    /**
     * Query operational status
     */
    isAuthenticated: async () => {
      return context.isAuthenticated;
    },
  };
}
