<script lang="ts">
  import ContentWrapper from "$lib/components/content-wrapper.svelte";
  import Navbar from "$lib/components/navbar.svelte";
  import Skip from "$lib/a11y/skip.svelte";
  import type { ActionData, PageData } from "./$types";

  let { data, form }: { data: PageData; form: ActionData } = $props();
  let date_format = new Intl.DateTimeFormat(undefined, {
    year: "numeric",
    month: "2-digit",
    day: "2-digit",
    hour: "2-digit",
    minute: "2-digit",
  });
</script>

<svelte:head>
  <title>Guestbook</title>
</svelte:head>

<ContentWrapper>
  <Skip />
  <Navbar />

  <form method="POST" name="guestbook">
    <div class="form-container">
      {#if form?.missing && form.name}
        <p class="form-error">No name provided!</p>
      {/if}
      {#if form?.missing && form.text}
        <p class="form-error">No text provided!</p>
      {/if}
      {#if form?.invalid && form.website}
        <p class="form-error">The website was invalid! (Missing https?)</p>
      {/if}
      {#if form?.invalid && form.too_long}
        <p class="form-error">Your text was too long!</p>
      {/if}

      <div class="small-form-container">
        <div>
          <label for="name_input">Name:</label>
          <input
            type="text"
            name="name"
            id="name_input"
            class="form-input form-textbox"
            style="width:150px"
            required
            maxlength="100"
            placeholder="Melody"
          />
        </div>
        <div>
          <label for="website_input">Website:</label>
          <input
            type="text"
            name="website"
            class="form-input form-textbox"
            style="width:300px"
            id="website_input"
            maxlength="100"
            placeholder="https://melody-is.gay"
          />
        </div>
      </div>
      <textarea
        name="text"
        class="form-big-textbox form-input form-textbox"
        required
        placeholder="Yep, this is a guestbook"
        minlength="1"
        maxlength="500"
      ></textarea>
      <!-- eugh the css syntax for this SUCKS -->
      <input
        type="submit"
        value="Submit"
        class="form-input submit-button"
        style="padding: 2px 8px 2px 8px"
      />
    </div>
  </form>

  <div id="main"></div>

  {#each data.entries as entry, index}
    <div class="guestbook-entry">
      <!-- no idea why span works... -->
      <span role="heading" aria-level="1">
        {entry.name}
      </span>

      {#if entry.website !== "" && entry.website !== null}
        <a
          href={entry.website}
          rel="external"
          aria-label="a globe with several meridans"
        >
          <img
            src="/logos/globe.svg"
            class="website-globe"
            alt="a globe with several meridans"
            style="vertical-align:text-bottom"
          />
        </a>
      {/if}

      <!-- silly hack to get rid of commas -->
      <span class="entry-date" role="heading" aria-level="1">
        {date_format.format(entry.date).replace(/,/, "")}
      </span>
      <p>{entry.text}</p>
    </div>
    {#if index < data.entries.length - 1}
      <hr />
    {/if}
  {/each}

  <div style="height:20px"></div>
</ContentWrapper>

<style>
  .guestbook-entry {
    margin-top: 16px;
  }
  span {
    font-size: 32px;
    color: white;
  }
  .website-globe {
    filter: invert(100%);
    width: 32px;
  }
  .entry-date {
    float: right;
  }

  .form-error {
    color: #fc5050;
  }
  .small-form-container {
    display: flex;
    flex-direction: row;
    gap: 16px;
  }
  .form-container {
    padding-top: 50px;
    gap: 16px;

    display: flex;
    flex-wrap: wrap;
    flex-direction: column;
    align-items: center;
    justify-content: center;
  }

  .form-input {
    color: whitesmoke;
    font-size: 18px;

    border: min(6vw, 8px) solid;
    border-image: url("/border.png") 4 stretch;
    image-rendering: pixelated;
    background: rgba(3, 1, 4, 0.78);
    background-clip: content-box;
    padding: 0;
  }
  .form-textbox {
    filter: brightness(100%);
  }
  .form-textbox:hover {
    filter: brightness(150%);
  }
  .form-textbox:focus {
    filter: brightness(160%);
  }
  .form-big-textbox {
    /* TODO make nicer */
    width: 600px;
    height: 100px;
    font-size: 20px;
    resize: vertical;
  }

  .submit-button {
    filter: brightness(100%);
  }
  .submit-button:hover {
    filter: brightness(150%);
  }
  .submit-button:active {
    filter: brightness(60%);
  }

  label {
    font-size: 24px;
    color: white;
  }

  p {
    font-size: 24px;
    font-weight: 300;
  }
</style>
