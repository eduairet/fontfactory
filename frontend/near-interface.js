export class FontFactory {
    constructor({ contractId, walletToUse }) {
        this.contractId = contractId;
        this.wallet = walletToUse;
    }

    async getFontid() {
        return await this.wallet.viewMethod({
            contractId: this.contractId,
            method: 'get_font_id',
        });
    }

    async createCustomFont(fontid) {
        return await this.wallet.callMethod({
            contractId: this.contractId,
            method: 'create_custom_font',
            args: { fontid: fontid },
        });
    }
}
