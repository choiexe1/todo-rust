<script lang="ts">
	import { invalidateAll } from '$app/navigation';
	import type { PageData } from './$types';

	export let data: PageData;
	let todos = data.todos;

	async function deleteTodo(id: number) {
		await fetch(`http://0.0.0.0:3000/delete/${id}`, { method: 'POST' });
		invalidateAll();
	}

	async function updateTodo(todo: Todo) {
		await fetch(
			`http://0.0.0.0:3000/update?id=${todo.id}&description=${todo.description}&done=${todo.done}`
		);
	}
</script>

<div class="container mx-auto mt-16">
	<h1 class="h1 text-center">Todos</h1>

	<div class="max-w-screen-md mx-auto">
		<form action="http://0.0.0.0:3000/create" method="POST">
			<input
				class="input p-4 my-8"
				name="description"
				type="text"
				placeholder="할 일을 작성하고 Enter를 입력하세요."
				autocomplete="off"
			/>
		</form>

		<div class="space-y-4">
			{#each todos as todo}
				<div class="flex items-center justify-between p-4 bg-surface-800 rounded-lg gap-4">
					<input
						class="checkbox"
						type="checkbox"
						bind:checked={todo.done}
						on:change={() => updateTodo(todo)}
					/>
					<input class="input" type="text" bind:value={todo.description} disabled={todo.done} />

					<div class="flex gap-2">
						<button class="btn variant-filled-secondary" on:click={() => updateTodo(todo)}
							>수정</button
						>

						<button class="btn variant-filled-primary" on:click={() => deleteTodo(todo.id)}
							>삭제</button
						>
					</div>
				</div>
			{/each}
		</div>
	</div>
</div>
