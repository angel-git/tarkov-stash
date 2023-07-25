import crypto from 'crypto';
import { Time } from '~/helper/time';

export class Hash {
    private readonly time: Time;

    constructor() {
        this.time = new Time();
    }

    /**
     * Create a 24 character id using the sha256 algorithm + current timestamp
     * @returns 24 character hash
     */
    public generate(): string {
        const shasum = crypto.createHash('sha256');
        const time = Math.random() * this.time.getTimestamp();

        shasum.update(time.toString());
        return shasum.digest('hex').substring(0, 24);
    }

    public generateMd5ForData(data: string): string {
        return this.generateHashForData('md5', data);
    }

    public generateSha1ForData(data: string): string {
        return this.generateHashForData('sha1', data);
    }

    /**
     * Create a hash for the data parameter
     * @param algorithm algorithm to use to hash
     * @param data data to be hashed
     * @returns hash value
     */
    public generateHashForData(algorithm: string, data: crypto.BinaryLike): string {
        const hashSum = crypto.createHash(algorithm);
        hashSum.update(data);
        return hashSum.digest('hex');
    }
}
