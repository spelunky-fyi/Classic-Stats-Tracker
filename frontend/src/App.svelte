<script lang="ts">
  import EnemyIcon from "./EnemyIcon.svelte";
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
    <div>
      <StatIcon collection="misc_deaths" name="rock" />
      <StatIcon collection="misc_deaths" name="explosion" />
      <StatIcon collection="misc_deaths" name="crushed" />
      <StatIcon collection="misc_deaths" name="long_fall" />
      <StatIcon collection="misc_deaths" name="spikes" />
      <!-- <StatIcon collection="misc_deaths" name="boulder" /> -->
    </div>
    <div>
      <StatIcon collection="misc_deaths" name="arrow_trap" />
      <StatIcon collection="misc_deaths" name="spear_trap" />
      <StatIcon collection="misc_deaths" name="smash_trap" />
      <StatIcon collection="misc_deaths" name="ceiling_trap" />
      <StatIcon collection="misc_deaths" name="pit" />
      <StatIcon collection="misc_deaths" name="lava" />
    </div>
    <div>&nbsp;</div>
    <div>
      <EnemyIcon name="bat" showDeaths={false} />
      <EnemyIcon name="snake" />
    </div>
    <div>
      <EnemyIcon name="spider" />
      <EnemyIcon name="giant_spider" />
    </div>
    <div>
      <EnemyIcon name="caveman" />
      <EnemyIcon name="skeleton" />
    </div>
    <div>
      <EnemyIcon name="zombie" showDeaths={false} />
      <EnemyIcon name="vampire" showDeaths={false} />
    </div>
    <div>
      <EnemyIcon name="frog" />
      <EnemyIcon name="monkey" showDeaths={false} />
    </div>
    <div>
      <EnemyIcon name="fire_frog" />
      <EnemyIcon name="mantrap" />
    </div>
    <div>
      <EnemyIcon name="piranha" />
      <EnemyIcon name="megamouth" showDeaths={false} />
    </div>
    <div>
      <EnemyIcon name="yeti" />
      <EnemyIcon name="yeti_king" />
    </div>
    <div>
      <EnemyIcon name="alien" />
      <EnemyIcon name="ufo" />
    </div>
    <div>
      <EnemyIcon name="alien_boss" />
      <EnemyIcon name="hawkman" />
    </div>
    <div>
      <EnemyIcon name="tomb_lord" />
      <EnemyIcon name="shopkeeper" />
    </div>
    <div>
      <EnemyIcon name="magma_man" showKills={false} />
      <EnemyIcon name="olmec" />
    </div>
    <div>
      <EnemyIcon name="ghost" showKills={false} />
    </div>
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
