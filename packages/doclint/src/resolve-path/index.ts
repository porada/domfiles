import { fileURLToPath } from 'node:url';

/**
 * Returns an absolute filesystem path using a file URL as the base
 */
export default function resolvePath(path: string, base: string | URL): string {
	return fileURLToPath(new URL(path, base));
}
