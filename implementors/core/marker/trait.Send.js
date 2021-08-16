(function() {var implementors = {};
implementors["txrx"] = [{"text":"impl&lt;Left, Right&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx/adaptors/and/struct.And.html\" title=\"struct txrx::adaptors::and::And\">And</a>&lt;Left, Right&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Left: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Right: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["txrx::adaptors::and::And"]},{"text":"impl&lt;Input, Func&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx/adaptors/and_then/struct.AndThen.html\" title=\"struct txrx::adaptors::and_then::AndThen\">AndThen</a>&lt;Input, Func&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Func: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Input: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["txrx::adaptors::and_then::AndThen"]},{"text":"impl&lt;Input, Func, NextReceiver&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx/adaptors/and_then/struct.AndThenReceiver.html\" title=\"struct txrx::adaptors::and_then::AndThenReceiver\">AndThenReceiver</a>&lt;Input, Func, NextReceiver&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Func: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Input: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;NextReceiver: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["txrx::adaptors::and_then::AndThenReceiver"]},{"text":"impl&lt;InputSender, Func&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx/adaptors/bulk/struct.Bulk.html\" title=\"struct txrx::adaptors::bulk::Bulk\">Bulk</a>&lt;InputSender, Func&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Func: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;InputSender: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["txrx::adaptors::bulk::Bulk"]},{"text":"impl&lt;R, Func, Scheduler&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx/adaptors/bulk/struct.BulkReceiver.html\" title=\"struct txrx::adaptors::bulk::BulkReceiver\">BulkReceiver</a>&lt;R, Func, Scheduler&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Func: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Scheduler: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["txrx::adaptors::bulk::BulkReceiver"]},{"text":"impl&lt;Func, Next&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx/adaptors/bulk/struct.BulkWork.html\" title=\"struct txrx::adaptors::bulk::BulkWork\">BulkWork</a>&lt;Func, Next&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Func: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html\" title=\"trait core::marker::Sync\">Sync</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Next: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;Next as <a class=\"trait\" href=\"txrx/traits/receiver/trait.Receiver.html\" title=\"trait txrx::traits::receiver::Receiver\">Receiver</a>&gt;::<a class=\"type\" href=\"txrx/traits/receiver/trait.Receiver.html#associatedtype.Input\" title=\"type txrx::traits::receiver::Receiver::Input\">Input</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["txrx::adaptors::bulk::BulkWork"]},{"text":"impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx/adaptors/ensure_started/struct.ReceiverType.html\" title=\"struct txrx::adaptors::ensure_started::ReceiverType\">ReceiverType</a>&lt;S&gt;","synthetic":true,"types":["txrx::adaptors::ensure_started::ReceiverType"]},{"text":"impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx/adaptors/ensure_started/struct.EnsureStarted.html\" title=\"struct txrx::adaptors::ensure_started::EnsureStarted\">EnsureStarted</a>&lt;S&gt;","synthetic":true,"types":["txrx::adaptors::ensure_started::EnsureStarted"]},{"text":"impl&lt;S, F&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx/adaptors/map/struct.Map.html\" title=\"struct txrx::adaptors::map::Map\">Map</a>&lt;S, F&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;F: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["txrx::adaptors::map::Map"]},{"text":"impl&lt;Input, Recv, Func&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx/adaptors/map/struct.Receiver.html\" title=\"struct txrx::adaptors::map::Receiver\">Receiver</a>&lt;Input, Recv, Func&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Func: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Input: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;Recv: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["txrx::adaptors::map::Receiver"]},{"text":"impl&lt;SenderT, SchedulerT&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx/adaptors/transfer/struct.Transfer.html\" title=\"struct txrx::adaptors::transfer::Transfer\">Transfer</a>&lt;SenderT, SchedulerT&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;SchedulerT: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;SenderT: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["txrx::adaptors::transfer::Transfer"]},{"text":"impl&lt;Next&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx/adaptors/transfer/struct.TransferJob.html\" title=\"struct txrx::adaptors::transfer::TransferJob\">TransferJob</a>&lt;Next&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Next: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;Next as <a class=\"trait\" href=\"txrx/traits/receiver/trait.Receiver.html\" title=\"trait txrx::traits::receiver::Receiver\">Receiver</a>&gt;::<a class=\"type\" href=\"txrx/traits/receiver/trait.Receiver.html#associatedtype.Error\" title=\"type txrx::traits::receiver::Receiver::Error\">Error</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;&lt;Next as <a class=\"trait\" href=\"txrx/traits/receiver/trait.Receiver.html\" title=\"trait txrx::traits::receiver::Receiver\">Receiver</a>&gt;::<a class=\"type\" href=\"txrx/traits/receiver/trait.Receiver.html#associatedtype.Input\" title=\"type txrx::traits::receiver::Receiver::Input\">Input</a>: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["txrx::adaptors::transfer::TransferJob"]},{"text":"impl&lt;Next, SchedT&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx/adaptors/transfer/struct.TransferReceiver.html\" title=\"struct txrx::adaptors::transfer::TransferReceiver\">TransferReceiver</a>&lt;Next, SchedT&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;Next: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;SchedT: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["txrx::adaptors::transfer::TransferReceiver"]},{"text":"impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx/consumers/into_awaitable/struct.Awaitable.html\" title=\"struct txrx::consumers::into_awaitable::Awaitable\">Awaitable</a>&lt;S&gt;","synthetic":true,"types":["txrx::consumers::into_awaitable::Awaitable"]},{"text":"impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx/consumers/start_detached/struct.SinkFor.html\" title=\"struct txrx::consumers::start_detached::SinkFor\">SinkFor</a>&lt;S&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;S: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["txrx::consumers::start_detached::SinkFor"]},{"text":"impl&lt;V, E&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"enum\" href=\"txrx/consumers/sync_wait/enum.Result.html\" title=\"enum txrx::consumers::sync_wait::Result\">Result</a>&lt;V, E&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;E: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;V: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["txrx::consumers::sync_wait::Result"]},{"text":"impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx/consumers/sync_wait/struct.State.html\" title=\"struct txrx::consumers::sync_wait::State\">State</a>&lt;S&gt;","synthetic":true,"types":["txrx::consumers::sync_wait::State"]},{"text":"impl&lt;S&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx/consumers/sync_wait/struct.Recv.html\" title=\"struct txrx::consumers::sync_wait::Recv\">Recv</a>&lt;S&gt;","synthetic":true,"types":["txrx::consumers::sync_wait::Recv"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx/factories/done/struct.Done.html\" title=\"struct txrx::factories::done::Done\">Done</a>","synthetic":true,"types":["txrx::factories::done::Done"]},{"text":"impl&lt;T&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx/factories/just/struct.Just.html\" title=\"struct txrx::factories::just::Just\">Just</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["txrx::factories::just::Just"]},{"text":"impl&lt;SchedulerT, SenderT&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx/factories/on/struct.On.html\" title=\"struct txrx::factories::on::On\">On</a>&lt;SchedulerT, SenderT&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;SchedulerT: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;SenderT: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["txrx::factories::on::On"]},{"text":"impl&lt;SenderT, ReceiverT&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx/factories/on/struct.OnWork.html\" title=\"struct txrx::factories::on::OnWork\">OnWork</a>&lt;SenderT, ReceiverT&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;ReceiverT: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;SenderT: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["txrx::factories::on::OnWork"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx/manual_executor/struct.ManualExecutor.html\" title=\"struct txrx::manual_executor::ManualExecutor\">ManualExecutor</a>","synthetic":true,"types":["txrx::manual_executor::ManualExecutor"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx/manual_executor/struct.Runner.html\" title=\"struct txrx::manual_executor::Runner\">Runner</a>","synthetic":true,"types":["txrx::manual_executor::Runner"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx/manual_executor/struct.ScheduledSender.html\" title=\"struct txrx::manual_executor::ScheduledSender\">ScheduledSender</a>","synthetic":true,"types":["txrx::manual_executor::ScheduledSender"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx/manual_executor/struct.Scheduler.html\" title=\"struct txrx::manual_executor::Scheduler\">Scheduler</a>","synthetic":true,"types":["txrx::manual_executor::Scheduler"]},{"text":"impl&lt;R&gt; <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx/utility/struct.ReceiverRef.html\" title=\"struct txrx::utility::ReceiverRef\">ReceiverRef</a>&lt;R&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;R: <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a>,&nbsp;</span>","synthetic":true,"types":["txrx::utility::ReceiverRef"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx/struct.ImmediateScheduler.html\" title=\"struct txrx::ImmediateScheduler\">ImmediateScheduler</a>","synthetic":true,"types":["txrx::immediate_scheduler::ImmediateScheduler"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx/test/struct.ManualTrigger.html\" title=\"struct txrx::test::ManualTrigger\">ManualTrigger</a>","synthetic":true,"types":["txrx::test::ManualTrigger"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx/test/struct.ManualSender.html\" title=\"struct txrx::test::ManualSender\">ManualSender</a>","synthetic":true,"types":["txrx::test::ManualSender"]}];
implementors["txrx_rayon"] = [{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx_rayon/struct.PoolScheduler.html\" title=\"struct txrx_rayon::PoolScheduler\">PoolScheduler</a>","synthetic":true,"types":["txrx_rayon::PoolScheduler"]},{"text":"impl <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/marker/trait.Send.html\" title=\"trait core::marker::Send\">Send</a> for <a class=\"struct\" href=\"txrx_rayon/struct.GlobalScheduler.html\" title=\"struct txrx_rayon::GlobalScheduler\">GlobalScheduler</a>","synthetic":true,"types":["txrx_rayon::GlobalScheduler"]}];
if (window.register_implementors) {window.register_implementors(implementors);} else {window.pending_implementors = implementors;}})()