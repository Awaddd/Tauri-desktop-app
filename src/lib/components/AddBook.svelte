<script lang="ts">
  import { addBook } from "../../services/books";

  let title = "";
  let author = "";
  let error: string | null;

  async function onSubmit(){
    ({ error } = await addBook({ title, author }));
  }
</script>

<div>
  <form class="row" on:submit|preventDefault={onSubmit}>
    <input id="greet-input" placeholder="Enter a book title..." bind:value={title} />
    <input id="greet-input" placeholder="Enter the author..." bind:value={author} />
    <button type="submit">Add Book</button>
  </form>

  {#if error}
    <p>{error}</p>
  {/if}
</div>