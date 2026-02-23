const assert = require('assert');
const wareki = require('./index.js');

function runTests() {
    console.log("Running Node.js tests...");

    // to_wareki normal
    const w1 = wareki.toWareki(2026, 2, 23);
    assert.strictEqual(w1.eraName, "令和");
    assert.strictEqual(w1.year, 8);

    const w2 = wareki.toWareki(1989, 1, 8);
    assert.strictEqual(w2.eraName, "平成");
    assert.strictEqual(w2.year, 1);

    // to_wareki string representation test
    assert.strictEqual(`${w1.eraName}${w1.year}年`, "令和8年");

    // from_wareki normal
    const d1 = wareki.fromWareki("令和", 8, 2, 23);
    assert.strictEqual(d1, "2026-02-23");

    // from_wareki abbreviations
    assert.strictEqual(wareki.fromWareki("令", 8, 2, 23), "2026-02-23");
    assert.strictEqual(wareki.fromWareki("r", 8, 2, 23), "2026-02-23");
    assert.strictEqual(wareki.fromWareki("R", 8, 2, 23), "2026-02-23");

    // leap year
    const leapDate = wareki.fromWareki("令和", 6, 2, 29);
    assert.strictEqual(leapDate, "2024-02-29");

    // invalid leap year
    assert.throws(() => {
        wareki.fromWareki("令和", 5, 2, 29);
    }, { code: 'InvalidArg' });

    // out of range before Meiji
    assert.throws(() => {
        wareki.toWareki(1868, 1, 24);
    }, { code: 'InvalidArg' });

    console.log("All Node.js tests passed! \u2705");
}

runTests();
