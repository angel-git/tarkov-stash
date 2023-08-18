const ROUBLE = '5449016a4bdc2d6f028b456f';
const DOLLAR = '5696686a4bdc2da3298b456a';
const EURO = '569668774bdc2da2298b4568';
export const ITEMS_TEMPLATE_UPDATABLE = [ROUBLE, DOLLAR, EURO];


export const hexStringToCssColor = (hex: string, opacity = 1): string => {
    // we don't allow custom colors for currencies
    if (ITEMS_TEMPLATE_UPDATABLE.includes(hex)) {
        return 'none';
    }
    // Extract three 8-character segments from the hex string
    const r = hex.substring(0, 8);
    const g = hex.substring(8, 16);
    const b = hex.substring(16, 24);

    // Convert the segments to integers
    const rInt = parseInt(r, 16) % 256;
    const gInt = parseInt(g, 16) % 256;
    const bInt = parseInt(b, 16) % 256;

    // Format the RGB values as a CSS color
    return `rgba(${rInt}, ${gInt}, ${bInt}, ${opacity})`;
}
