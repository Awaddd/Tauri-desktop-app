<script lang="ts">
  import { onMount } from 'svelte';
  import { getBooks } from '../../services/books'

  let loading = false;
  let data: Book[] = []
  let error: string;

  onMount(() => {
    async function load() {
      ({ data, error } = await getBooks());


      loading = false;
    }

    loading = true;
    load();
  })
</script>

{#if data && data.length > 0}
  {#each data as book}
    <li class="list-none text-center">{book.title} by {book.author} ({book.isbn})</li>
  {/each}
{/if}

{#if loading}
  <p class="text-center">Loading...</p>
{/if}

{#if error}
  <p>{error}</p>
{/if}
