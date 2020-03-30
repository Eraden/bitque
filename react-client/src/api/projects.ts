import { request } from './index';

export const updateProject = form =>
    request({ path: `/project/${ form.get('id') }`, form });
