<script lang="ts">
  import { onMount } from 'svelte';
  import { getBooks } from '../../services/books'

  let data: Book[] = []
  let error: string;

  onMount(() => {
    async function load() {
      ({ data, error } = await getBooks());
    }

    load();
  })
</script>

<div>
  {#if data && data.length > 0}
    {#each data as book}
      <li>{book.title} by {book.author} ({book.isbn})</li>
    {/each}
  {/if}

  {#if error}
    <p>{error}</p>
  {/if}
</div>