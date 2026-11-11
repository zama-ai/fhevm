import { MISSING_SECTION_ERROR } from '../../model/execution.js';

/**
 * A step used a module on a profile without the sections it needs (P-002), typically a scenario
 * that forgot `spec.targets` and runs on `none`. The engine reports it as a configuration error.
 */
export class MissingProfileSectionError extends Error {
  constructor(
    readonly profileName: string,
    readonly sections: string[],
  ) {
    const plural = sections.length > 1;
    super(
      `profile '${profileName}' has no ${sections.join(', ')} section${plural ? 's' : ''}; declare spec.targets for this scenario so that it only runs on profiles that have ${plural ? 'them' : 'one'}`,
    );
    this.name = MISSING_SECTION_ERROR;
  }
}
