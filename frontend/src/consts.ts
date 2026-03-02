export type CreateShortUrlRequest = {
    url: string;
    expiresAt?: number;
    maxClicks?: number;
};

export type CreateShortUrlResponse = {
    id: string;
    shortUrl: string;
    longUrl: string;
    clicks: number;
    expiresAt?: number;
    maxClicks?: number;
    disabled: boolean;
    lastClickedAt?: number;
    createdAt: number;
};

export type GetUrlResponse = {
    longUrl: string;
};

export type GetUrlInfoResponse = {
    id: string;
    shortUrl: string;
    longUrl: string;
    clicks: number;
    expiresAt?: number;
    maxClicks?: number;
    disabled: boolean;
    lastClickedAt?: number;
    createdAt: number;
};
