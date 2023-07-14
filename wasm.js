import { scheduled } from './pkg/rustwasm_namnsdag.js';


try {
  await scheduled({ cron: 'adddddsd', scheduledTime: 1637873341148, type: 'scheduled' });
} catch (err) {
  console.error(err)
}
