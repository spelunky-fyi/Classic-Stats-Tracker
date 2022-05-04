<script lang="ts">
  import EnemyList1 from "./EnemyList1.svelte";
  import EnemyList2 from "./EnemyList2.svelte";
  import MiscDeaths1 from "./MiscDeaths1.svelte";
  import MiscDeaths2 from "./MiscDeaths2.svelte";
  import StatIcon from "./StatIcon.svelte";
  import store from "./stores";

  let connected = false;

  store.connected.subscribe((value) => {
    console.log(value);
    connected = value;
  });
</script>

<main>
  {#if connected}
    <div>
      <StatIcon name="crates_opened" />
      <StatIcon name="chests_opened" />
      <StatIcon name="idols_grabbed" />
      <StatIcon name="idols_converted" />
    </div>
    <div>
      <StatIcon name="damsels_grabbed" />
      <StatIcon name="kisses_bought" />
      <StatIcon name="damsels_bought" />
      <StatIcon name="damsels_saved" />
      <StatIcon name="damsels_killed" />
    </div>
    <div>
      <StatIcon name="items_bought" />
      <StatIcon name="items_stolen" />
      <StatIcon name="dice_games_played" />
      <StatIcon name="dice_games_won" />
      <StatIcon name="dice_games_lost" />
    </div>
    <div>&nbsp;</div>
    <div>
      <StatIcon collection="level_deaths" name="level_1" />
      <StatIcon collection="level_deaths" name="level_2" />
      <StatIcon collection="level_deaths" name="level_3" />
      <StatIcon collection="level_deaths" name="level_4" />
    </div>
    <div>
      <StatIcon collection="level_deaths" name="level_5" />
      <StatIcon collection="level_deaths" name="level_6" />
      <StatIcon collection="level_deaths" name="level_7" />
      <StatIcon collection="level_deaths" name="level_8" />
    </div>
    <div>
      <StatIcon collection="level_deaths" name="level_9" />
      <StatIcon collection="level_deaths" name="level_10" />
      <StatIcon collection="level_deaths" name="level_11" />
      <StatIcon collection="level_deaths" name="level_12" />
    </div>
    <div>
      <StatIcon collection="level_deaths" name="level_13" />
      <StatIcon collection="level_deaths" name="level_14" />
      <StatIcon collection="level_deaths" name="level_15" />
      <StatIcon collection="level_deaths" name="level_16" />
    </div>
    <div>&nbsp;</div>
    {#if store.ruleset === 1}
      <MiscDeaths1 />
    {:else}
      <MiscDeaths2 />
    {/if}

    <div>&nbsp;</div>
    {#if store.ruleset === 1}
      <EnemyList1 />
    {:else}
      <EnemyList2 />
    {/if}
  {:else}
    <div class="connecting">Connecting...</div>
  {/if}
</main>

<style>
  main {
    padding: 0.1em;
    margin: 0 auto;
  }

  div.connecting {
    color: red;
    font-weight: bold;
    font-size: x-large;
  }
  @media (min-width: 640px) {
    main {
      max-width: none;
    }
  }
</style>
