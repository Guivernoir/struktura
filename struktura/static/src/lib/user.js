/**
 * @file user.js
 * @description User profile and project management
 * Mission objective: Personnel records and strategic asset management
 */

export function createUserModule(requestHandler) {
  return {
    /**
     * Retrieve operative dossier
     */
    me: () => requestHandler("/user/profile/me"),

    /**
     * Update operative credentials
     */
    update: (data) =>
      requestHandler("/user/profile/update", {
        method: "PUT",
        body: data,
      }),

    /**
     * Query operational statistics
     */
    stats: () => requestHandler("/user/stats/me"),

    /**
     * Project management subsystem
     */
    projects: {
      /**
       * Retrieve all active operations
       */
      getAll: () => requestHandler("/user/projects"),

      /**
       * Initialize new strategic operation
       */
      create: (data) =>
        requestHandler("/user/projects", {
          method: "POST",
          body: data,
        }),
    },
  };
}
