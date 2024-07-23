<script lang="ts">
  export let amount: number;
  export let max: number;

  function handleKeyUp() {
    if (amount < 1) {
      amount = 1;
    } else if (amount > max) {
      amount = max;
    }
  }

  function setAmountToMax() {
    amount = max || 1;
  }
</script>

<div class="amount-container">
  <input type="number" on:keyup={handleKeyUp} bind:value={amount} min="1" {max} />
  <button on:click={setAmountToMax} disabled={amount === max}>Set to max</button>
</div>

<style>
  .amount-container {
    display: flex;
    gap: 8px;
    justify-content: center;
  }

  input {
    padding: 8px;
    border: 1px solid var(--color-text);
    background-color: var(--color-background);
    color: var(--color-text);
  }

  input:focus {
    outline: none;
    border: 1px solid var(--color-highlight);
  }

  button {
    background-color: var(--color-background);
    color: var(--color-secondary);
    border: 1px solid;
    border-color: var(--color-secondary);
    border-radius: 2px;
    cursor: pointer;
    transition-duration: 0.2s;
  }

  button:hover,
  button:focus {
    outline: none;
    background-color: var(--color-secondary);
    color: var(--color-background);
  }

  button:disabled,
  button:disabled:hover {
    cursor: default;
    opacity: 0.2;
    background-color: var(--color-background);
    color: var(--color-secondary);
  }
</style>
