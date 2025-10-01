declare module "moesif-browser-js" {
  interface MoesifOptions {
    applicationId: string;
    [key: string]: any;
  }

  interface UserMetadata {
    email?: string;
    firstName?: string;
    lastName?: string;
    [key: string]: any;
  }

  interface CompanyMetadata {
    [key: string]: any;
  }

  interface Moesif {
    init(options: MoesifOptions): void;
    identifyUser(userId: string, metadata?: UserMetadata): void;
    identifyCompany(
      companyId: string,
      metadata?: CompanyMetadata,
      companyDomain?: string
    ): void;
    track(actionName: string, metadata?: object): void;
    identifySession(sessionToken: string): void;
    reset(): void;
    start(): void;
    stop(): void;
    useWeb3(web3: any): boolean;
  }

  const moesif: Moesif;
  export default moesif;
}
