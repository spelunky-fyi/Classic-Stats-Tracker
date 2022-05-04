<script lang="ts">
  import EnemyIcon from "./EnemyIcon.svelte";
</script>

<main>
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
</main>

<style>
  main {
    padding: 0.1em;
    margin: 0 auto;
  }
</style>
