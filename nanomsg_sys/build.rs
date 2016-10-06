extern crate gcc;

fn main() {
	let mut base_config = gcc::Config::new();
	base_config.include("nanomsg/include/")
		.define("USE_NUM_NONE", Some("1"));

	if cfg!(target_os = "macos") {
		base_config.define("NN_HAVE_CLANG", Some("1"));
		base_config.define("NN_HAVE_OSX", Some("1"));
		base_config.define("HAVE_KQUEUE", Some("1"));
		base_config.define("NN_USE_KQUEUE", Some("1"));

	}
	if cfg!(target_os = "linux") {
		base_config.define("_GNU_SOURCE", Some("1"));
		base_config.define("NN_HAVE_GCC", Some("1"));
		base_config.define("NN_HAVE_LINUX", Some("1"));
		base_config.define("HAVE_EVENTFD", Some("1"));
		base_config.define("NN_HAVE_EVENTFD", Some("1"));
		base_config.define("HAVE_PIPE2", Some("1"));
		base_config.define("NN_HAVE_PIPE2", Some("1"));
		base_config.define("NN_HAVE_CLOCK_MONOTONIC", Some("1"));
		base_config.define("HAVE_CLOCK_GETTIME", Some("1"));
		base_config.define("HAVE_EPOLL_CREATE", Some("1"));
		base_config.define("NN_USE_EPOLL", Some("1"));
		base_config.define("HAVE_ACCEPT4", Some("1"));
		base_config.define("NN_HAVE_ACCEPT4", Some("1"));
		base_config.define("NN_HAVE_GETADDRINFO_A", Some("1"));
		base_config.define("NN_HAVE_SOCKETPAIR", Some("1"));
		base_config.define("NN_USE_EVENTFD", Some("1"));
	}

	if cfg!(unix) {
		base_config.define("HAVE_PTHREAD_PRIO_INHERIT", Some("1"));
		base_config.define("HAVE_SYS_TYPES_H", Some("1"));
		base_config.define("HAVE_SYS_STAT_H", Some("1"));
		base_config.define("HAVE_STRING_H", Some("1"));
		base_config.define("HAVE_STDLIB_H", Some("1"));
		base_config.define("HAVE_STRINGS_H", Some("1"));
		base_config.define("HAVE_MEMORY_H", Some("1"));
		base_config.define("HAVE_INTTYPES_H", Some("1"));
		base_config.define("HAVE_UNISTD_H", Some("1"));
		base_config.define("HAVE_STDINT_H", Some("1"));
		base_config.define("HAVE_NETINET_IN_H", Some("1"));
		base_config.define("HAVE_NETDB_H", Some("1"));
		base_config.define("HAVE_ARPA_INET_H", Some("1"));
		base_config.define("HAVE_UNISTD_H", Some("1"));
		base_config.define("HAVE_SYS_SOCKET_H", Some("1"));
		base_config.define("HAVE_SYS_IOCTL_H", Some("1"));
		base_config.define("HAVE_STDINT_H", Some("1"));
		base_config.define("NN_HAVE_STDINT", Some("1"));
		base_config.define("HAVE_PIPE", Some("1"));
		base_config.define("NN_HAVE_PIPE", Some("1"));
		base_config.define("HAVE_POLL", Some("1"));
		base_config.define("NN_HAVE_POLL", Some("1"));
		base_config.define("NN_HAVE_SOCKETPAIR", Some("1"));
		base_config.define("NN_HAVE_SEMAPHORE", Some("1"));
		base_config.define("NN_HAVE_MSG_CONTROL", Some("1"));
		base_config.define("NN_USE_PIPE", Some("1"));
		base_config.define("HAVE_DLFCN_H", Some("1"));
	}

	if cfg!(windows) {
		link("Mswsock", false);
		base_config.define("NN_NO_EXPORTS", Some("1"));
		base_config.define("NN_STATIC_LIB", Some("1"));
		base_config.define("NN_HAVE_WINDOWS", Some("1"));
		base_config.define("NN_USE_WINSOCK", Some("1"));
		base_config.define("_CRT_SECURE_NO_WARNINGS)", Some("1"));
	}

	base_config.file("nanomsg/src/core/ep.c");
	base_config.file("nanomsg/src/core/epbase.c");
	base_config.file("nanomsg/src/core/global.c");
	base_config.file("nanomsg/src/core/pipe.c");
	base_config.file("nanomsg/src/core/poll.c");
	base_config.file("nanomsg/src/core/sock.c");
	base_config.file("nanomsg/src/core/sockbase.c");
	base_config.file("nanomsg/src/core/symbol.c");

	base_config.file("nanomsg/src/devices/device.c");

	base_config.file("nanomsg/src/aio/ctx.c");
	base_config.file("nanomsg/src/aio/fsm.c");
	base_config.file("nanomsg/src/aio/poller.c");
	base_config.file("nanomsg/src/aio/pool.c");
	base_config.file("nanomsg/src/aio/timer.c");
	base_config.file("nanomsg/src/aio/timerset.c");
	base_config.file("nanomsg/src/aio/usock.c");
	base_config.file("nanomsg/src/aio/worker.c");

	base_config.file("nanomsg/src/utils/alloc.c");
	base_config.file("nanomsg/src/utils/atomic.c");
	base_config.file("nanomsg/src/utils/chunk.c");
	base_config.file("nanomsg/src/utils/chunkref.c");
	base_config.file("nanomsg/src/utils/clock.c");
	base_config.file("nanomsg/src/utils/condvar.c");
	base_config.file("nanomsg/src/utils/closefd.c");
	base_config.file("nanomsg/src/utils/efd.c");
	base_config.file("nanomsg/src/utils/err.c");
	base_config.file("nanomsg/src/utils/hash.c");
	base_config.file("nanomsg/src/utils/list.c");
	base_config.file("nanomsg/src/utils/msg.c");
	base_config.file("nanomsg/src/utils/mutex.c");
	base_config.file("nanomsg/src/utils/once.c");
	base_config.file("nanomsg/src/utils/queue.c");
	base_config.file("nanomsg/src/utils/random.c");
	base_config.file("nanomsg/src/utils/sem.c");
	base_config.file("nanomsg/src/utils/sleep.c");
	base_config.file("nanomsg/src/utils/stopwatch.c");
	base_config.file("nanomsg/src/utils/thread.c");
	base_config.file("nanomsg/src/utils/wire.c");

	base_config.file("nanomsg/src/protocols/utils/dist.c");
	base_config.file("nanomsg/src/protocols/utils/excl.c");
	base_config.file("nanomsg/src/protocols/utils/fq.c");
	base_config.file("nanomsg/src/protocols/utils/lb.c");
	base_config.file("nanomsg/src/protocols/utils/priolist.c");

	base_config.file("nanomsg/src/protocols/bus/bus.c");
	base_config.file("nanomsg/src/protocols/bus/xbus.c");

	base_config.file("nanomsg/src/protocols/pipeline/push.c");
	base_config.file("nanomsg/src/protocols/pipeline/pull.c");
	base_config.file("nanomsg/src/protocols/pipeline/xpull.c");
	base_config.file("nanomsg/src/protocols/pipeline/xpush.c");

	base_config.file("nanomsg/src/protocols/pair/pair.c");
	base_config.file("nanomsg/src/protocols/pair/xpair.c");

	base_config.file("nanomsg/src/protocols/pubsub/pub.c");
	base_config.file("nanomsg/src/protocols/pubsub/sub.c");
	base_config.file("nanomsg/src/protocols/pubsub/trie.c");
	base_config.file("nanomsg/src/protocols/pubsub/xpub.c");
	base_config.file("nanomsg/src/protocols/pubsub/xsub.c");

	base_config.file("nanomsg/src/protocols/reqrep/req.c");
	base_config.file("nanomsg/src/protocols/reqrep/rep.c");
	base_config.file("nanomsg/src/protocols/reqrep/task.c");
	base_config.file("nanomsg/src/protocols/reqrep/xrep.c");
	base_config.file("nanomsg/src/protocols/reqrep/xreq.c");

	base_config.file("nanomsg/src/protocols/survey/respondent.c");
	base_config.file("nanomsg/src/protocols/survey/surveyor.c");
	base_config.file("nanomsg/src/protocols/survey/xrespondent.c");
	base_config.file("nanomsg/src/protocols/survey/xsurveyor.c");

	base_config.file("nanomsg/src/transports/utils/backoff.c");
	base_config.file("nanomsg/src/transports/utils/dns.c");
	base_config.file("nanomsg/src/transports/utils/iface.c");
	base_config.file("nanomsg/src/transports/utils/literal.c");
	base_config.file("nanomsg/src/transports/utils/port.c");
	base_config.file("nanomsg/src/transports/utils/streamhdr.c");
	base_config.file("nanomsg/src/transports/utils/base64.c");

	base_config.file("nanomsg/src/transports/inproc/binproc.c");
	base_config.file("nanomsg/src/transports/inproc/cinproc.c");
	base_config.file("nanomsg/src/transports/inproc/inproc.c");
	base_config.file("nanomsg/src/transports/inproc/ins.c");
	base_config.file("nanomsg/src/transports/inproc/msgqueue.c");
	base_config.file("nanomsg/src/transports/inproc/sinproc.c");

	base_config.file("nanomsg/src/transports/ipc/aipc.c");
	base_config.file("nanomsg/src/transports/ipc/bipc.c");
	base_config.file("nanomsg/src/transports/ipc/cipc.c");
	base_config.file("nanomsg/src/transports/ipc/ipc.c");
	base_config.file("nanomsg/src/transports/ipc/sipc.c");

	base_config.file("nanomsg/src/transports/tcp/atcp.c");
	base_config.file("nanomsg/src/transports/tcp/btcp.c");
	base_config.file("nanomsg/src/transports/tcp/ctcp.c");
	base_config.file("nanomsg/src/transports/tcp/stcp.c");
	base_config.file("nanomsg/src/transports/tcp/tcp.c");

	base_config.file("nanomsg/src/transports/ws/aws.c");
	base_config.file("nanomsg/src/transports/ws/bws.c");
	base_config.file("nanomsg/src/transports/ws/cws.c");
	base_config.file("nanomsg/src/transports/ws/sws.c");
	base_config.file("nanomsg/src/transports/ws/ws.c");
	base_config.file("nanomsg/src/transports/ws/ws_handshake.c");
	base_config.file("nanomsg/src/transports/ws/sha1.c");


	base_config.compile("libnanomsg.a");
}

pub fn link(name: &str, bundled: bool) {
    use std::env::var;
    let target = var("TARGET").unwrap();
    let target: Vec<_> = target.split('-').collect();
    if target.get(2) == Some(&"windows") {
        println!("cargo:rustc-link-lib=dylib={}", name);
        if bundled && target.get(3) == Some(&"gnu") {
            let dir = var("CARGO_MANIFEST_DIR").unwrap();
            println!("cargo:rustc-link-search=native={}/{}", dir, target[0]);
        }
    }
}