cd backend
if( cargo build --release)
then
	mv target/release/libedit.so ../frontend/
	cd ..
	echo 'python3 frontend/ui.py' > sprite_maker
	chmod a+x sprite_maker
fi
