<script lang="ts">
  import Books from './lib/components/Books.svelte';
  import AddBook from './lib/components/AddBook.svelte'
</script>

<main class="container">
  <h1>Bookio</h1>

  <div class="row mt-xl">
    <Books />
  </div>

  <div class="row mt-xl">
    <AddBook />
  </div>
</main>
