declare module "moesif-browser-js" {
  /**
   * Options for initializing the Moesif Browser JS SDK.
   */
  export interface MoesifOptions {
    /**
     * Your Publishable Moesif Application Id.
     */
    applicationId: string;
    /**
     * Persistance option for anonymousId and other session info.
     * 'localStorage' (default): writes to localStorage and replicates to a cookie.
     * 'cookie': persists to cookies only.
     * 'none': nothing will be persisted. Not recommended.
     */
    persistence?: "localStorage" | "cookie" | "none";
    /**
     * Set to true to enable debug logging.
     */
    debug?: boolean;
  }

  export interface Moesif {
    /**
     * Initialize the SDK with your Publishable Application Id and other options.
     * This method must be called before any other methods.
     * @param options - Configuration options for the Moesif SDK.
     */
    init(options: MoesifOptions): void;

    /**
     * When a user logs in, identify them with your userId.
     * You can also add custom metadata.
     * @param userId - The user's unique identifier.
     * @param metadata - Optional metadata about the user.
     */
    identifyUser(userId: string, metadata?: object): void;

    /**
     * Similar to identifyUser, but for tracking companies or accounts.
     * @param companyId - The company's unique identifier.
     * @param metadata - Optional metadata about the company.
     * @param companyDomain - Optional company website or email domain for enrichment.
     */
    identifyCompany(
      companyId: string,
      metadata?: object,
      companyDomain?: string
    ): void;

    /**
     * Track user actions such as "clicked sign up" or "made a purchase".
     * @param eventName - The name of the action or event.
     * @param metadata - Optional metadata related to the event.
     */
    track(eventName: string, metadata?: object): void;

    /**
     * Override the automatically tracked browser session with a specific session token.
     * @param sessionToken - The session token to use.
     */
    identifySession(sessionToken: string): void;

    /**
     * Clears any saved userId, companyId, and other device context.
     * Call this when a user logs out of your web app.
     */
    reset(): void;

    /**
     * Start logging outgoing API calls made via AJAX.
     */
    start(): void;

    /**
     * Stops logging AJAX API calls.
     */
    stop(): void;

    /**
     * Sets the web3 JSON-RPC to use the provided web3 object.
     * @param web3 - The web3 object to use.
     * @returns True if successful.
     */
    useWeb3(web3: any): boolean;
  }

  const moesif: Moesif;
  export default moesif;
}
