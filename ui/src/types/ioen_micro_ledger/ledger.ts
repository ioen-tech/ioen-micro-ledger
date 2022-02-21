export interface Consumer {
    address: string;
    postcode: number;
}

export interface Producer {
    address: string;
    postcode: number;
    method: string;
}

export interface SupplyAgreement {
    from: number;
    to: number;
    price: number;
}

export interface SupplyBlock {
    quality: number;
    from: number;
    to: number;
    price: number;
}

export interface Bill {
    address: string;
    dueDate: number;
    status: string;
    notes: string;
}

